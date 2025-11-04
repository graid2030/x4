use axum::{extract::State, http::StatusCode, Json};

use crate::models::{PilotInfo, PlayerAsset, PlayerNpc, PlayerPropertyResponse};

use super::common::AppState;

/// Get pilot info
pub async fn get_pilot(State(state): State<AppState>) -> Result<Json<PilotInfo>, StatusCode> {
    state.save_repository.ensure_latest().await?;

    let pilot = state
        .save_repository
        .get_pilot()
        .await?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(pilot))
}

/// Get all player-owned assets, NPCs, and wing links
pub async fn get_player_property(
    State(state): State<AppState>,
) -> Result<Json<PlayerPropertyResponse>, StatusCode> {
    state.save_repository.ensure_latest().await?;

    let mut assets: Vec<PlayerAsset> = state.save_repository.get_player_assets().await?;
    let npcs: Vec<PlayerNpc> = state.save_repository.get_player_npcs().await?;

    let game_guard = state.game_data.read().await;
    let game = match game_guard.as_ref() {
        Some(g) => g,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    // Enrich ships with cargo capacity/type and speed from cache
    for a in &mut assets {
        if a.class.contains("ship") {
            if let Some(mac) = &a.macro_name {
                let key = mac.to_lowercase();
                if let Some(sm) = game.ship_meta.get(&key) {
                    if a.cargo_capacity.is_none() {
                        a.cargo_capacity = sm.cargo_capacity;
                    }
                    if a.cargo_type.is_none() {
                        a.cargo_type = sm.cargo_type.clone();
                    }
                    if a.speed.is_none() {
                        a.speed = sm.max_speed;
                    }
                }
            }
        }
    }

    Ok(Json(PlayerPropertyResponse { assets, npcs }))
}
