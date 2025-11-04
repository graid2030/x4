use axum::{extract::State, http::StatusCode, Json};

use crate::models::{PilotInfo, PlayerPropertyResponse, PlayerAsset, PlayerNpc};

use super::common::AppState;

/// Get pilot info
pub async fn get_pilot(State(state): State<AppState>) -> Result<Json<PilotInfo>, StatusCode> {
    // Always compute fresh from save so credits are up to date
    let (game, save_path) = {
        let gd = state.game_data.read().await;
        let sd = state.save_data.read().await;
        match (gd.as_ref(), sd.as_ref()) {
            (Some(g), Some(s)) => (g.clone(), s.save_path.clone()),
            _ => return Err(StatusCode::BAD_REQUEST),
        }
    };

    let xml = crate::parsers::save_xml::load_save_file(&save_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let pilot = crate::parsers::pilot_xml::extract_pilot_info(&xml, &game.localization)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Cache latest pilot in memory
    {
        let mut sdw = state.save_data.write().await;
        if let Some(ref mut s) = *sdw { s.pilot = Some(pilot.clone()); }
    }

    Ok(Json(pilot))
}

/// Get all player-owned assets, NPCs, and wing links
pub async fn get_player_property(State(state): State<AppState>) -> Result<Json<PlayerPropertyResponse>, StatusCode> {
    let game = state.game_data.read().await;
    let save = state.save_data.read().await;
    let (game, save) = match (game.as_ref(), save.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let xml = crate::parsers::save_xml::load_save_file(&save.save_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut assets: Vec<PlayerAsset> = crate::parsers::assets::extract_player_assets(
        &xml,
        &game.sector_names,
        &game.component_names,
        &game.localization,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let npcs: Vec<PlayerNpc> = crate::parsers::assets::extract_player_npcs(&xml, &game.localization)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Enrich ships with cargo capacity/type and speed from cache
    for a in &mut assets {
        if a.class.contains("ship") {
            if let Some(mac) = &a.macro_name {
                let key = mac.to_lowercase();
                if let Some(sm) = game.ship_meta.get(&key) {
                    if a.cargo_capacity.is_none() { a.cargo_capacity = sm.cargo_capacity; }
                    if a.cargo_type.is_none() { a.cargo_type = sm.cargo_type.clone(); }
                    if a.speed.is_none() { a.speed = sm.max_speed; }
                }
            }
        }
    }

    Ok(Json(PlayerPropertyResponse { assets, npcs }))
}
