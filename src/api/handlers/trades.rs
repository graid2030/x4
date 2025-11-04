use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::models::{TradeFilters, TradeOffer};
use crate::parsers::game_xml::resolve_name;
use crate::services::ArbitrageService;

use super::common::AppState;

/// Get trade offers with filters
pub async fn get_trade_offers(
    State(state): State<AppState>,
    Json(filters): Json<TradeFilters>,
) -> Result<Json<Vec<TradeOffer>>, StatusCode> {
    state.save_repository.ensure_latest().await?;

    let game_data = state.game_data.read().await;
    let save_data = state.save_data.read().await;

    let (game, _save) = match (game_data.as_ref(), save_data.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let sector_filter = filters.sectors.as_ref().map(|s| s.as_slice());
    let filtered_trades = state.save_repository.get_trades(sector_filter).await?;

    // Calculate arbitrage
    let mut offers = ArbitrageService::calculate_arbitrage(&filtered_trades, &game.wares, &filters);

    // Filter by selected wares
    if let Some(selected_wares) = &filters.wares {
        offers.retain(|offer| selected_wares.contains(&offer.ware));
    }

    // Populate human-readable ware names
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

    Ok(Json(offers))
}

#[derive(Deserialize)]
pub struct WareTradesRequest {
    pub ware_id: String,
    pub sectors: Option<Vec<String>>,
    pub cargo_volume: Option<f64>,
    pub same_sector_only: bool,
}

/// Get all trades for a specific ware
pub async fn get_ware_trades(
    State(state): State<AppState>,
    Json(req): Json<WareTradesRequest>,
) -> Result<Json<Vec<TradeOffer>>, StatusCode> {
    state.save_repository.ensure_latest().await?;

    let game_data = state.game_data.read().await;
    let save_data = state.save_data.read().await;

    let (game, _save) = match (game_data.as_ref(), save_data.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let sector_filter = req.sectors.as_ref().map(|s| s.as_slice());
    let filtered_trades = state.save_repository.get_trades(sector_filter).await?;

    // Calculate arbitrage with group_by_ware=false to get all trades
    let filters = TradeFilters {
        sectors: req.sectors.clone(),
        wares: Some(vec![req.ware_id.clone()]),
        ship_id: None,
        cargo_volume: req.cargo_volume,
        group_by_ware: false, // Important: get all trades, not grouped
        same_sector_only: req.same_sector_only,
    };

    let mut offers = ArbitrageService::calculate_arbitrage(&filtered_trades, &game.wares, &filters);

    // Filter by the specific ware
    offers.retain(|offer| offer.ware == req.ware_id);

    // Populate human-readable ware names
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

    Ok(Json(offers))
}
