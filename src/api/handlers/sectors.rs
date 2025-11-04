use axum::{extract::State, http::StatusCode, Json};

use crate::models::{Sector, SectorListItem, SectorStats, SectorsListResponse};

use super::common::AppState;

/// Get available sectors
pub async fn get_sectors(State(state): State<AppState>) -> Result<Json<Vec<Sector>>, StatusCode> {
    state.save_repository.ensure_latest().await?;

    let save_data = state.save_data.read().await;
    match save_data.as_ref() {
        Some(data) => Ok(Json(data.sectors.clone())),
        None => Err(StatusCode::BAD_REQUEST),
    }
}

/// Get sectors list with aggregated statistics
pub async fn get_sectors_list(
    State(state): State<AppState>,
) -> Result<Json<SectorsListResponse>, StatusCode> {
    state.save_repository.ensure_latest().await?;

    let game_data = state.game_data.read().await;
    let save_data = state.save_data.read().await;

    let (game, save) = match (game_data.as_ref(), save_data.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let all_trades = state
        .save_repository
        .get_all_trades()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let station_counts = state
        .save_repository
        .get_station_counts()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Use faction names from game data (extracted from libraries/factions.xml)
    let faction_names = &game.faction_names;

    // Build sector list items
    let mut sectors_list: Vec<SectorListItem> = Vec::new();
    let mut total_stations = 0;
    let mut player_owned_sectors = 0;
    let mut discovered_count = 0;

    for sector in &save.sectors {
        let sector_code = &sector.code;

        // Count offers by type
        let (buy_offers, sell_offers) = if let Some(trades) = all_trades.get(sector_code) {
            let buys = trades
                .iter()
                .filter(|t| matches!(t.trade_type, crate::models::TradeType::Buy))
                .count();
            let sells = trades
                .iter()
                .filter(|t| matches!(t.trade_type, crate::models::TradeType::Sell))
                .count();
            (buys, sells)
        } else {
            (0, 0)
        };

        // Get station counts
        let (station_count, player_stations) =
            station_counts.get(sector_code).copied().unwrap_or((0, 0));

        total_stations += station_count;

        // Check if player owns sector
        let is_player_owned = sector
            .owner
            .as_ref()
            .map(|o| o == "player")
            .unwrap_or(false);
        if is_player_owned {
            player_owned_sectors += 1;
        }

        // Discovered (for now assume all discovered, will add knownto later)
        let discovered = true;
        if discovered {
            discovered_count += 1;
        }

        // Map owner to name (fallback to raw owner ID if not in map)
        let owner_name = sector.owner.as_ref().map(|o| {
            faction_names
                .get(o.as_str())
                .map(|n| n.to_string())
                .unwrap_or_else(|| o.clone())
        });

        sectors_list.push(SectorListItem {
            code: sector.code.clone(),
            name: sector.name.clone(),
            macro_name: sector.macro_name.clone(),
            owner: sector.owner.clone(),
            owner_name,
            contested: false, // TODO: extract from save
            station_count,
            player_stations,
            buy_offers,
            sell_offers,
            total_trades: buy_offers + sell_offers,
            resource_types: vec![], // TODO: Phase 4 - resource areas
            discovered,
        });
    }

    // Sort by name
    sectors_list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let stats = SectorStats {
        total_sectors: save.sectors.len(),
        player_owned: player_owned_sectors,
        discovered: discovered_count,
        total_stations,
    };

    Ok(Json(SectorsListResponse {
        sectors: sectors_list,
        stats,
    }))
}
