use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;

use crate::parsers::game_xml::resolve_name;

use super::common::AppState;

#[derive(Serialize)]
pub struct WareInfo {
    pub id: String,
    pub name: String,
}

/// Get available wares
pub async fn get_wares(State(state): State<AppState>) -> Result<Json<Vec<WareInfo>>, StatusCode> {
    let game_data = state.game_data.read().await;
    match game_data.as_ref() {
        Some(data) => {
            let mut items: Vec<WareInfo> = data
                .wares
                .values()
                .map(|m| WareInfo {
                    id: m.id.clone(),
                    name: m
                        .name_ref
                        .as_ref()
                        .map(|r| resolve_name(r, &data.localization))
                        .unwrap_or_else(|| m.id.clone()),
                })
                .collect();
            items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            ;

            Ok(Json(items))
        }
        None => Err(StatusCode::BAD_REQUEST),
    }
}
