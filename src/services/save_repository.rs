use axum::http::StatusCode;
use std::collections::{hash_map::DefaultHasher, HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::api::handlers::SaveData;
use crate::models::{StationCount, StationSummary, StationWare};
use crate::parsers::save_xml::{
    extract_all_trades, extract_sectors, extract_station_data, load_save_file, StationParseResult,
};
use crate::services::GameDataCache;

pub struct SaveDataRepository {
    game_data: Arc<RwLock<Option<GameDataCache>>>,
    save_data: Arc<RwLock<Option<SaveData>>>,
    refresh_lock: Mutex<()>,
}

impl SaveDataRepository {
    pub fn new(
        game_data: Arc<RwLock<Option<GameDataCache>>>,
        save_data: Arc<RwLock<Option<SaveData>>>,
    ) -> Self {
        Self {
            game_data,
            save_data,
            refresh_lock: Mutex::new(()),
        }
    }

    async fn refresh_if_needed(&self) -> Result<(), StatusCode> {
        let (save_path, cached_modified, caches_missing) = {
            let guard = self.save_data.read().await;
            let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            (
                save.save_path.clone(),
                save.last_modified,
                save.trades_by_sector.is_empty()
                    || save.station_counts.is_empty()
                    || save.stations.is_empty(),
            )
        };

        let metadata = fs::metadata(&save_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let modified = metadata.modified().ok();

        let mut needs_refresh = caches_missing;
        if !needs_refresh {
            needs_refresh = match (modified, cached_modified) {
                (Some(new), Some(old)) => new != old,
                (Some(_), None) => true,
                (None, Some(_)) => true,
                _ => false,
            };
        }

        if !needs_refresh {
            return Ok(());
        }

        let _refresh_guard = self.refresh_lock.lock().await;

        let (save_path, cached_modified, caches_missing) = {
            let guard = self.save_data.read().await;
            let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
            (
                save.save_path.clone(),
                save.last_modified,
                save.trades_by_sector.is_empty()
                    || save.station_counts.is_empty()
                    || save.stations.is_empty(),
            )
        };

        let metadata = fs::metadata(&save_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let modified = metadata.modified().ok();

        let mut needs_refresh = caches_missing;
        if !needs_refresh {
            needs_refresh = match (modified, cached_modified) {
                (Some(new), Some(old)) => new != old,
                (Some(_), None) => true,
                (None, Some(_)) => true,
                _ => false,
            };
        }

        if !needs_refresh {
            return Ok(());
        }

        let save_content =
            load_save_file(&save_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut hasher = DefaultHasher::new();
        save_content.hash(&mut hasher);
        let content_hash = format!("{:016x}", hasher.finish());

        let (sectors, trades, StationParseResult { stations, counts }) = {
            let guard = self.game_data.read().await;
            let game = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;

            let sectors = extract_sectors(
                &save_content,
                &game.sector_names,
                &game.component_names,
                &game.sector_code_names,
                &game.localization,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let trades = extract_all_trades(
                &save_content,
                &game.sector_names,
                &game.component_names,
                &game.localization,
                &sectors,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let station_parse = extract_station_data(
                &save_content,
                &game.component_names,
                &game.faction_names,
                &game.localization,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            (sectors, trades, station_parse)
        };

        let mut station_lookup: HashMap<String, StationSummary> = HashMap::new();
        for summary in &stations {
            station_lookup.insert(summary.code.clone(), summary.clone());
        }

        {
            let mut guard = self.save_data.write().await;
            let save = guard.as_mut().ok_or(StatusCode::BAD_REQUEST)?;
            save.sectors = sectors;
            save.trades_by_sector = trades;
            save.station_counts = counts;
            save.stations = stations;
            save.station_lookup = station_lookup;
            save.last_modified = modified;
            save.content_hash = Some(content_hash);
        }

        Ok(())
    }

    pub async fn ensure_latest(&self) -> Result<(), StatusCode> {
        self.refresh_if_needed().await
    }

    pub async fn get_trades(
        &self,
        sectors: Option<&[String]>,
    ) -> Result<HashMap<String, Vec<StationWare>>, StatusCode> {
        self.refresh_if_needed().await?;
        let guard = self.save_data.read().await;
        let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;

        if let Some(sector_filter) = sectors {
            let filter: HashSet<&str> = sector_filter.iter().map(|s| s.as_str()).collect();
            let filtered = save
                .trades_by_sector
                .iter()
                .filter(|(code, _)| filter.contains(code.as_str()))
                .map(|(code, trades)| (code.clone(), trades.clone()))
                .collect();
            Ok(filtered)
        } else {
            Ok(save.trades_by_sector.clone())
        }
    }

    pub async fn get_all_trades(&self) -> Result<HashMap<String, Vec<StationWare>>, StatusCode> {
        self.get_trades(None).await
    }

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
}
