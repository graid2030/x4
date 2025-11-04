use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use std::collections::HashMap;

use crate::models::{TradeFilters, TradeOffer, StationWare};
use crate::parsers::game_xml::resolve_name;
use crate::parsers::save_xml::{extract_all_trades, load_save_file};
use crate::services::ArbitrageService;

use super::common::AppState;

/// Get trade offers with filters
pub async fn get_trade_offers(
    State(state): State<AppState>,
    Json(filters): Json<TradeFilters>,
) -> Result<Json<Vec<TradeOffer>>, StatusCode> {
    let game_data = state.game_data.read().await;
    let save_data = state.save_data.read().await;

    let (game, save) = match (game_data.as_ref(), save_data.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Reload save file
    let save_content = load_save_file(&save.save_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract all trades by sector
    let all_trades = extract_all_trades(
        &save_content,
        &game.sector_names,
        &game.component_names,
        &game.localization,
        &save.sectors,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Filter by selected sectors
    let filtered_trades: HashMap<String, Vec<StationWare>> = if let Some(selected_sectors) = &filters.sectors {
        all_trades
            .into_iter()
            .filter(|(code, _)| selected_sectors.contains(code))
            .collect()
    } else {
        all_trades
    };

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
    let game_data = state.game_data.read().await;
    let save_data = state.save_data.read().await;

    let (game, save) = match (game_data.as_ref(), save_data.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Reload save file
    let save_content = load_save_file(&save.save_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract all trades by sector
    let all_trades = extract_all_trades(
        &save_content,
        &game.sector_names,
        &game.component_names,
        &game.localization,
        &save.sectors,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Filter by selected sectors
    let filtered_trades: HashMap<String, Vec<StationWare>> = if let Some(selected_sectors) = &req.sectors {
        all_trades
            .into_iter()
            .filter(|(code, _)| selected_sectors.contains(code))
            .collect()
    } else {
        all_trades
    };

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
