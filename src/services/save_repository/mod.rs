mod maps;
mod players;
mod sectors;
mod trades;

use axum::http::StatusCode;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::fs;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::api::handlers::SaveData;
use crate::models::StationSummary;
use crate::parsers::pilot_xml::extract_pilot_info;
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

        let save_string =
            load_save_file(&save_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut hasher = DefaultHasher::new();
        save_string.hash(&mut hasher);
        let content_hash = format!("{:016x}", hasher.finish());
        let save_content = Arc::new(save_string);

        let (sectors, trades, StationParseResult { stations, counts }, pilot) = {
            let guard = self.game_data.read().await;
            let game = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;

            let sectors = extract_sectors(
                save_content.as_ref(),
                &game.sector_names,
                &game.component_names,
                &game.sector_code_names,
                &game.localization,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let trades = extract_all_trades(
                save_content.as_ref(),
                &game.sector_names,
                &game.component_names,
                &game.localization,
                &sectors,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let station_parse = extract_station_data(
                save_content.as_ref(),
                &game.component_names,
                &game.faction_names,
                &game.localization,
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let pilot = extract_pilot_info(save_content.as_ref(), &game.localization).ok();

            (sectors, trades, station_parse, pilot)
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
            save.pilot = pilot;
            save.save_content = Some(save_content);
            save.player_assets = None;
            save.player_npcs = None;
            save.sector_maps.clear();
            save.stations_by_sector.clear();
        }

        Ok(())
    }

    pub async fn ensure_latest(&self) -> Result<(), StatusCode> {
        self.refresh_if_needed().await
    }
}
