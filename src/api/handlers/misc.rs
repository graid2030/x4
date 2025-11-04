use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::models::SectorMapData;
use crate::parsers::{extract_sector_map, load_save_file};
use crate::services::GameDataCache;

use super::common::AppState;

#[derive(Serialize)]
pub struct LastPathsResponse {
    pub game_path: Option<String>,
    pub saves_dir: Option<String>,
}

/// Return last-used paths from cache (if present)
pub async fn get_last_paths() -> Result<Json<LastPathsResponse>, StatusCode> {
    let cache_path = "x4-cache.json";
    match GameDataCache::load_from_file(cache_path) {
        Ok(cache) => {
            let resp = match cache.last_paths {
                Some(lp) => LastPathsResponse { game_path: Some(lp.game_path), saves_dir: Some(lp.saves_dir) },
                None => LastPathsResponse { game_path: None, saves_dir: None },
            };
            Ok(Json(resp))
        }
        Err(_) => Ok(Json(LastPathsResponse { game_path: None, saves_dir: None })),
    }
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub initialized: bool,
}

/// Check if the server is already initialized in memory
pub async fn get_status(State(state): State<AppState>) -> Result<Json<StatusResponse>, StatusCode> {
    let gd = state.game_data.read().await;
    let sd = state.save_data.read().await;
    let initialized = gd.is_some() && sd.is_some();
    Ok(Json(StatusResponse { initialized }))
}

#[derive(Deserialize)]
pub struct ListSavesRequest {
    pub saves_dir: String,
}

#[derive(Serialize)]
pub struct SaveFileInfo {
    pub filename: String,
    pub modified: Option<String>,
    #[serde(skip)]
    pub modified_timestamp: Option<u64>,
}

/// List save files in a directory
pub async fn list_save_files(
    Json(req): Json<ListSavesRequest>,
) -> Result<Json<Vec<SaveFileInfo>>, StatusCode> {
    use std::fs;
    use std::time::SystemTime;

    let dir_path = std::path::Path::new(&req.saves_dir);
    if !dir_path.exists() || !dir_path.is_dir() {
        return Ok(Json(vec![]));
    }

    let mut saves = Vec::new();
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(filename) = entry.file_name().to_str() {
                        if filename.ends_with(".xml.gz") || filename.ends_with(".xml") {
                            let (modified_str, modified_ts) = entry.metadata()
                                .ok()
                                .and_then(|m| m.modified().ok())
                                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                                .map(|d| {
                                    let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(d.as_secs() as i64, 0);
                                    let formatted = datetime.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default();
                                    (Some(formatted), Some(d.as_secs()))
                                })
                                .unwrap_or((None, None));

                            saves.push(SaveFileInfo {
                                filename: filename.to_string(),
                                modified: modified_str,
                                modified_timestamp: modified_ts,
                            });
                        }
                    }
                }
            }
        }
    }

    // Sort by modification time (descending) to show newest saves first
    saves.sort_by(|a, b| {
        match (b.modified_timestamp, a.modified_timestamp) {
            (Some(t1), Some(t2)) => t1.cmp(&t2),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => b.filename.cmp(&a.filename),
        }
    });

    Ok(Json(saves))
}

#[derive(Deserialize)]
pub struct SectorMapRequest {
    pub sector_code: String,
}

/// Get sector map data
pub async fn get_sector_map(
    State(state): State<AppState>,
    Json(req): Json<SectorMapRequest>,
) -> Result<Json<SectorMapData>, StatusCode> {
    eprintln!("[API] get_sector_map called with sector_code: '{}'", req.sector_code);

    state.save_repository.ensure_latest().await?;

    let game_data = state.game_data.read().await;
    let save_data = state.save_data.read().await;

    let (game, save) = match (game_data.as_ref(), save_data.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Find sector by name or code (req.sector_code might be either)
    let sector = save
        .sectors
        .iter()
        .find(|s| s.code == req.sector_code || s.name == req.sector_code)
        .ok_or(StatusCode::NOT_FOUND)?;

    let actual_code = &sector.code;
    let sector_owner = sector.owner.clone();

    eprintln!("[API] Found sector: name='{}', code='{}', owner={:?}", sector.name, actual_code, sector_owner);

    // Reload save file
    let save_content = load_save_file(&save.save_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract sector map data
    let map_data = extract_sector_map(
        &save_content,
        actual_code,
        &game.sector_names,
        &game.component_names,
        &game.localization,
        sector_owner,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(map_data))
}
