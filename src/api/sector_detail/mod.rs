use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::collections::HashSet;

use crate::api::handlers::AppState;
use crate::models::{SectorDetailResponse, SectorDetailStats, TradeOfferDetail, TradeType};

/// Get detailed information about a specific sector
pub async fn get_sector_detail(
    State(state): State<AppState>,
    Path(sector_code): Path<String>,
) -> Result<Json<SectorDetailResponse>, StatusCode> {
    state.save_repository.ensure_latest().await?;

    let game_data = state.game_data.read().await;
    let save_data = state.save_data.read().await;

    let (game, save) = match (game_data.as_ref(), save_data.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Find sector by code
    let sector = save
        .sectors
        .iter()
        .find(|s| s.code == sector_code)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Extract stations for this sector via repository cache
    let stations = state
        .save_repository
        .get_sector_stations(&sector_code)
        .await?;

    // Use cached trades for this sector
    let filter = [sector_code.clone()];
    let mut sector_trades = state.save_repository.get_trades(Some(&filter)).await?;
    let sector_trades = sector_trades.remove(&sector_code).unwrap_or_default();

    let mut trade_offers = Vec::new();
    let mut buy_count = 0;
    let mut sell_count = 0;
    let mut unique_wares = HashSet::new();

    for trade in sector_trades {
        let trade_type_str = match trade.trade_type {
            TradeType::Buy => {
                buy_count += 1;
                "buy"
            }
            TradeType::Sell => {
                sell_count += 1;
                "sell"
            }
        };

        unique_wares.insert(trade.ware.clone());

        // Resolve ware name from game metadata via localization
        let ware_name = game
            .wares
            .get(&trade.ware)
            .and_then(|w| w.name_ref.as_ref())
            .map(|name_ref| {
                use crate::parsers::game_xml::resolve_name;
                resolve_name(name_ref, &game.localization)
            })
            .unwrap_or_else(|| trade.ware.clone());

        trade_offers.push(TradeOfferDetail {
            ware_id: trade.ware.clone(),
            ware_name,
            trade_type: trade_type_str.to_string(),
            price: trade.price,
            amount: trade.amount,
            station_code: trade.station_code.clone(),
            station_name: trade.station_name.clone(),
        });
    }

    // Calculate stats
    let player_stations_count = stations
        .iter()
        .filter(|s| s.owner.as_ref().map(|o| o == "player").unwrap_or(false))
        .count();

    let stats = SectorDetailStats {
        total_stations: stations.len(),
        player_stations: player_stations_count,
        buy_offers: buy_count,
        sell_offers: sell_count,
        unique_wares: unique_wares.len(),
    };

    // Map owner to name
    let owner_name = sector.owner.as_ref().map(|o| {
        game.faction_names
            .get(o.as_str())
            .map(|n| n.to_string())
            .unwrap_or_else(|| o.clone())
    });

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
