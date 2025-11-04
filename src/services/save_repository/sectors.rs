use std::collections::HashMap;

use axum::http::StatusCode;

use crate::models::{StationCount, StationInfo, StationSummary};

use crate::parsers::save_xml::extract_stations_for_sector;

use super::SaveDataRepository;

impl SaveDataRepository {
    pub async fn get_station_counts(&self) -> Result<HashMap<String, StationCount>, StatusCode> {
        self.refresh_if_needed().await?;
        let guard = self.save_data.read().await;
        let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
        Ok(save.station_counts.clone())
    }

    #[allow(dead_code)]
    pub async fn list_stations(&self) -> Result<Vec<StationSummary>, StatusCode> {
        self.refresh_if_needed().await?;
        let guard = self.save_data.read().await;
        let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
        Ok(save.stations.clone())
    }

    #[allow(dead_code)]
    pub async fn get_station(&self, code: &str) -> Result<Option<StationSummary>, StatusCode> {
        self.refresh_if_needed().await?;
        let guard = self.save_data.read().await;
        let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
        Ok(save.station_lookup.get(code).cloned())
    }

    pub async fn get_sector_stations(
        &self,
        sector_code: &str,
    ) -> Result<Vec<StationInfo>, StatusCode> {
        self.refresh_if_needed().await?;

        {
            let guard = self.save_data.read().await;
            let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            if let Some(stations) = save.stations_by_sector.get(sector_code) {
                return Ok(stations.clone());
            }
        }

        let save_content = {
            let guard = self.save_data.read().await;
            let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            if let Some(stations) = save.stations_by_sector.get(sector_code) {
                return Ok(stations.clone());
            }
            save.save_content
                .clone()
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        };

        let stations = {
            let guard = self.game_data.read().await;
            let game = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            extract_stations_for_sector(
                save_content.as_ref(),
                sector_code,
                &game.component_names,
                &game.faction_names,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        };

        let mut guard = self.save_data.write().await;
        let save = guard.as_mut().ok_or(StatusCode::BAD_REQUEST)?;
        save.stations_by_sector
            .insert(sector_code.to_string(), stations.clone());
        Ok(stations)
    }
}
