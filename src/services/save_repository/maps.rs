use axum::http::StatusCode;

use crate::models::SectorMapData;
use crate::parsers::sector_map::extract_sector_map;

use super::SaveDataRepository;

impl SaveDataRepository {
    pub async fn get_sector_map(
        &self,
        sector_code: &str,
        sector_owner: Option<String>,
    ) -> Result<SectorMapData, StatusCode> {
        self.refresh_if_needed().await?;

        {
            let guard = self.save_data.read().await;
            let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            if let Some(map) = save.sector_maps.get(sector_code) {
                return Ok(map.clone());
            }
        }

        let save_content = {
            let guard = self.save_data.read().await;
            let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            if let Some(map) = save.sector_maps.get(sector_code) {
                return Ok(map.clone());
            }
            save.save_content
                .clone()
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        };

        let map = {
            let guard = self.game_data.read().await;
            let game = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            extract_sector_map(
                save_content.as_ref(),
                sector_code,
                &game.sector_names,
                &game.component_names,
                &game.localization,
                sector_owner,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        };

        let mut guard = self.save_data.write().await;
        let save = guard.as_mut().ok_or(StatusCode::BAD_REQUEST)?;
        save.sector_maps
            .insert(sector_code.to_string(), map.clone());
        Ok(map)
    }
}
