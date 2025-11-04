use axum::http::StatusCode;

use crate::models::{PilotInfo, PlayerAsset, PlayerNpc};
use crate::parsers::assets::{extract_player_assets, extract_player_npcs};

use super::SaveDataRepository;

impl SaveDataRepository {
    pub async fn get_pilot(&self) -> Result<Option<PilotInfo>, StatusCode> {
        self.refresh_if_needed().await?;
        let guard = self.save_data.read().await;
        let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
        Ok(save.pilot.clone())
    }

    pub async fn get_player_assets(&self) -> Result<Vec<PlayerAsset>, StatusCode> {
        self.refresh_if_needed().await?;

        {
            let guard = self.save_data.read().await;
            let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            if let Some(assets) = &save.player_assets {
                return Ok(assets.clone());
            }
        }

        let save_content = {
            let guard = self.save_data.read().await;
            let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            if let Some(assets) = &save.player_assets {
                return Ok(assets.clone());
            }
            save.save_content
                .clone()
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        };

        let assets = {
            let guard = self.game_data.read().await;
            let game = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            extract_player_assets(
                save_content.as_ref(),
                &game.sector_names,
                &game.component_names,
                &game.localization,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        };

        let mut guard = self.save_data.write().await;
        let save = guard.as_mut().ok_or(StatusCode::BAD_REQUEST)?;
        save.player_assets = Some(assets.clone());
        Ok(assets)
    }

    pub async fn get_player_npcs(&self) -> Result<Vec<PlayerNpc>, StatusCode> {
        self.refresh_if_needed().await?;

        {
            let guard = self.save_data.read().await;
            let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            if let Some(npcs) = &save.player_npcs {
                return Ok(npcs.clone());
            }
        }

        let save_content = {
            let guard = self.save_data.read().await;
            let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            if let Some(npcs) = &save.player_npcs {
                return Ok(npcs.clone());
            }
            save.save_content
                .clone()
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        };

        let npcs = {
            let guard = self.game_data.read().await;
            let game = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            extract_player_npcs(save_content.as_ref(), &game.localization)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        };

        let mut guard = self.save_data.write().await;
        let save = guard.as_mut().ok_or(StatusCode::BAD_REQUEST)?;
        save.player_npcs = Some(npcs.clone());
        Ok(npcs)
    }
}
