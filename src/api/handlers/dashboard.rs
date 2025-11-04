use axum::{extract::State, http::StatusCode, Json};

use crate::models::{
    DashboardResponse, DashboardPlayer, DashboardStats, DashboardRoute, DashboardSector,
    TradeFilters,
};
use crate::parsers::game_xml::resolve_name;
use crate::parsers::save_xml::{extract_all_trades, load_save_file};
use crate::parsers::pilot_xml::extract_pilot_info;
use crate::parsers::assets::{extract_player_assets, extract_player_npcs};
use crate::services::ArbitrageService;

use super::common::AppState;
use super::sectors::count_stations_per_sector;

/// Get dashboard data (player, stats, top routes, sector summary)
pub async fn get_dashboard(State(state): State<AppState>) -> Result<Json<DashboardResponse>, StatusCode> {
    let game_data = state.game_data.read().await;
    let save_data = state.save_data.read().await;

    let (game, save) = match (game_data.as_ref(), save_data.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => {
            eprintln!("Dashboard error: Game data or save data not initialized");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Load save file
    let save_content = load_save_file(&save.save_path)
        .map_err(|e| {
            eprintln!("Dashboard error loading save file: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Extract pilot info
    let pilot_info = extract_pilot_info(&save_content, &game.localization)
        .map_err(|e| {
            eprintln!("Dashboard error extracting pilot info: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Extract player assets for ship/station counts
    let assets = extract_player_assets(
        &save_content,
        &game.sector_names,
        &game.component_names,
        &game.localization,
    )
    .map_err(|e| {
        eprintln!("Dashboard error extracting player assets: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Count ships and stations
    let ships = assets.iter().filter(|a| a.class.contains("ship")).count();
    let stations = assets.iter().filter(|a| a.class == "station").count();

    // Extract NPCs count
    let npcs = extract_player_npcs(&save_content, &game.localization)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .len();

    // Build player info
    let player = DashboardPlayer {
        name: pilot_info.name.clone(),
        money: pilot_info.credits,
        location: pilot_info.location.clone(),
        game_time: 0, // TODO: Extract game time from save if available
    };

    // Build stats
    let stats = DashboardStats {
        sectors: save.sectors.len(),
        stations,
        ships,
        npcs,
    };

    // Calculate top 5 trade routes
    let all_trades = extract_all_trades(
        &save_content,
        &game.sector_names,
        &game.component_names,
        &game.localization,
        &save.sectors,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let filters = TradeFilters {
        sectors: None,
        wares: None,
        ship_id: None,
        cargo_volume: Some(5000.0),
        same_sector_only: false,
        group_by_ware: true, // Only show best route per ware
    };

    let mut offers = ArbitrageService::calculate_arbitrage(&all_trades, &game.wares, &filters);

    // Populate ware names
    for offer in &mut offers {
        if let Some(meta) = game.wares.get(&offer.ware) {
            if let Some(name_ref) = &meta.name_ref {
                offer.ware_name = Some(resolve_name(name_ref, &game.localization));
            } else {
                offer.ware_name = Some(offer.ware.clone());
            }
        } else {
            offer.ware_name = Some(offer.ware.clone());
        }
    }

    // Sort by profit desc and take top 5
    offers.sort_by(|a, b| b.total_profit.partial_cmp(&a.total_profit).unwrap_or(std::cmp::Ordering::Equal));
    let top_routes: Vec<DashboardRoute> = offers
        .into_iter()
        .take(5)
        .map(|offer| {
            // Debug logging to diagnose naming issues
            eprintln!("[Dashboard] Route: {} | Buy: station='{}' sector='{}' | Sell: station='{}' sector='{}'",
                offer.ware, offer.buy_station, offer.buy_sector, offer.sell_station, offer.sell_sector);

            DashboardRoute {
                ware: offer.ware.clone(),
                ware_name: offer.ware_name.clone().unwrap_or_else(|| offer.ware.clone()),
                buy_station: offer.buy_station.clone(),
                buy_sector: offer.buy_sector.clone(),
                sell_station: offer.sell_station.clone(),
                sell_sector: offer.sell_sector.clone(),
                profit: offer.total_profit,
                buy_price: offer.buy_price,
                sell_price: offer.sell_price,
                amount: offer.qty,
            }
        })
        .collect();

    // Build sector summary (top sectors with most stations)
    let station_counts = count_stations_per_sector(&save_content)?;
    let mut sector_summary: Vec<DashboardSector> = save
        .sectors
        .iter()
        .map(|s| {
            let (station_count, player_stations) = station_counts
                .get(&s.code)
                .copied()
                .unwrap_or((0, 0));

            let owner_name = s.owner.as_ref().map(|o| {
                game.faction_names
                    .get(o.as_str())
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| o.clone())
            });

            DashboardSector {
                code: s.code.clone(),
                name: s.name.clone(),
                owner: s.owner.clone(),
                owner_name,
                station_count,
                player_stations,
            }
        })
        .collect();

    // Sort by station count desc and take top 10
    sector_summary.sort_by(|a, b| b.station_count.cmp(&a.station_count));
    sector_summary.truncate(10);

    Ok(Json(DashboardResponse {
        player,
        stats,
        top_routes,
        sector_summary,
    }))
}
