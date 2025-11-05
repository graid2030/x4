mod helpers;
mod station_parser;

use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::collections::HashSet;

use crate::api::handlers::AppState;
use crate::models::{SectorDetailResponse, TradeOfferDetail, SectorDetailStats, TradeType};
use crate::parsers::{load_save_file, save_xml::extract_all_trades};
use station_parser::extract_stations_for_sector;

/// Get detailed information about a specific sector
pub async fn get_sector_detail(
    State(state): State<AppState>,
    Path(sector_code): Path<String>,
) -> Result<Json<SectorDetailResponse>, StatusCode> {
    let game_data = state.game_data.read().await;
    let save_data = state.save_data.read().await;

    let (game, save) = match (game_data.as_ref(), save_data.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Find sector by code
    let sector = save.sectors.iter()
        .find(|s| s.code == sector_code)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Load save file for station extraction
    let save_content = load_save_file(&save.save_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract stations for this sector
    let stations = extract_stations_for_sector(
        &save_content,
        &sector_code,
        game.component_names_map(),
        game.faction_names_map(),
    )?;

    // Extract all trades and filter by sector
    let all_trades = extract_all_trades(
        &save_content,
        game.sector_names_map(),
        game.component_names_map(),
        game.localization_map(),
        &save.sectors,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get trade offers for this sector
    let mut trade_offers = Vec::new();
    let mut buy_count = 0;
    let mut sell_count = 0;
    let mut unique_wares = HashSet::new();

    if let Some(sector_trades) = all_trades.get(&sector_code) {
        for trade in sector_trades {
            let trade_type_str = match trade.trade_type {
                TradeType::Buy => { buy_count += 1; "buy" },
                TradeType::Sell => { sell_count += 1; "sell" },
            };

            unique_wares.insert(trade.ware.clone());

            // Resolve ware name from game metadata via localization
            trade_offers.push(TradeOfferDetail {
                ware_id: trade.ware.clone(),
                ware_name: game.get_ware_display_name(&trade.ware),
                trade_type: trade_type_str.to_string(),
                price: trade.price,
                amount: trade.amount,
                station_code: trade.station_code.clone(),
                station_name: trade.station_name.clone(),
            });
        }
    }

    // Calculate stats
    let player_stations_count = stations.iter().filter(|s| s.owner.as_ref().map(|o| o == "player").unwrap_or(false)).count();

    let stats = SectorDetailStats {
        total_stations: stations.len(),
        player_stations: player_stations_count,
        buy_offers: buy_count,
        sell_offers: sell_count,
        unique_wares: unique_wares.len(),
    };

    // Map owner to name
    let owner_name = sector
        .owner
        .as_ref()
        .map(|o| game.get_faction_display_name(o));

    Ok(Json(SectorDetailResponse {
        code: sector.code.clone(),
        name: sector.name.clone(),
        macro_name: sector.macro_name.clone(),
        owner: sector.owner.clone(),
        owner_name,
        contested: false, // TODO: extract from save
        discovered: true, // TODO: extract from knownto
        stations,
        trade_offers,
        stats,
    }))
}
