use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::parsers::{extract_sectors, load_save_file};
use crate::services::{GameDataCache, SavedPaths};

use super::common::{AppState, SaveData};
use super::init_utils::{apply_sector_name_overrides, ensure_sector_name_coverage};

#[derive(Deserialize)]
pub struct InitRequest {
    pub game_path: String,
    pub saves_dir: String,
    pub selected_save: String,
    #[serde(default = "default_lang_id")]
    pub lang_id: String,
}

fn default_lang_id() -> String {
    "44".to_string()
}

#[derive(Serialize)]
pub struct InitResponse {
    pub success: bool,
    pub message: String,
}

/// Initialize the system with game and save paths
pub async fn init_handler(
    State(state): State<AppState>,
    Json(req): Json<InitRequest>,
) -> Result<Json<InitResponse>, StatusCode> {
    // Load or extract game data
    let cache_path = "x4-cache.json".to_string();
    let mut game_data = GameDataCache::load_or_extract(&req.game_path, &cache_path, &req.lang_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    apply_sector_name_overrides(&mut game_data);

    // Build full save file path
    let save_path = format!("{}\\{}", req.saves_dir, req.selected_save);

    // Load save file
    let save_content = load_save_file(&save_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // DIAG: localization presence (even when loaded from cache)
    let has_20004 = game_data.localization.contains_key("20004");
    let page20004_len = game_data
        .localization
        .get("20004")
        .map(|p| p.len())
        .unwrap_or(0);
    let sample_keys: Vec<String> = game_data
        .localization
        .get("20004")
        .map(|p| p.keys().take(3).cloned().collect())
        .unwrap_or_else(|| Vec::new());
    eprintln!(
        "DIAG LOC (cache/load): pages={} has20004={} page20004_entries={} sample={:?}",
        game_data.localization.len(),
        has_20004,
        page20004_len,
        sample_keys
    );

    ensure_sector_name_coverage(
        &mut game_data,
        &req.game_path,
        &req.lang_id,
        &save_content,
        &cache_path,
    )?;

    let sectors = extract_sectors(
        &save_content,
        &game_data.sector_names,
        &game_data.component_names,
        &game_data.sector_code_names,
        &game_data.localization,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = sectors.len();

    // Extract pilot info
    let pilot =
        crate::parsers::pilot_xml::extract_pilot_info(&save_content, &game_data.localization).ok();

    // Persist last-used paths into cache file
    game_data.last_paths = Some(SavedPaths {
        game_path: req.game_path.clone(),
        saves_dir: req.saves_dir.clone(),
    });
    let _ = game_data.save_to_file(&cache_path);

    // Store in state
    *state.game_data.write().await = Some(game_data);
    let save_content_arc = Arc::new(save_content);

    *state.save_data.write().await = Some(SaveData {
        sectors,
        save_path,
        pilot,
        last_modified: None,
        content_hash: None,
        trades_by_sector: HashMap::new(),
        station_counts: HashMap::new(),
        stations: Vec::new(),
        station_lookup: HashMap::new(),
        save_content: Some(save_content_arc),
        player_assets: None,
        player_npcs: None,
        sector_maps: HashMap::new(),
        stations_by_sector: HashMap::new(),
    });

    state
        .save_repository
        .ensure_latest()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(InitResponse {
        success: true,
        message: "Initialized successfully".to_string(),
    }))
}
