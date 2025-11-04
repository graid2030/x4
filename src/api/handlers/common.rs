use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

use crate::models::{
    PilotInfo, PlayerAsset, PlayerNpc, Sector, SectorMapData, StationCount, StationInfo,
    StationSummary, StationWare,
};
use crate::services::{GameDataCache, SaveDataRepository};

#[derive(Clone)]
pub struct AppState {
    pub game_data: Arc<RwLock<Option<GameDataCache>>>,
    pub save_data: Arc<RwLock<Option<SaveData>>>,
    pub save_repository: Arc<SaveDataRepository>,
}

#[derive(Clone)]
pub struct SaveData {
    pub sectors: Vec<Sector>,
    pub save_path: String,
    pub pilot: Option<PilotInfo>,
    pub last_modified: Option<SystemTime>,
    pub content_hash: Option<String>,
    pub trades_by_sector: HashMap<String, Vec<StationWare>>,
    pub station_counts: HashMap<String, StationCount>,
    pub stations: Vec<StationSummary>,
    pub station_lookup: HashMap<String, StationSummary>,
    pub save_content: Option<Arc<String>>,
    pub player_assets: Option<Vec<PlayerAsset>>,
    pub player_npcs: Option<Vec<PlayerNpc>>,
    pub sector_maps: HashMap<String, SectorMapData>,
    pub stations_by_sector: HashMap<String, Vec<StationInfo>>,
}
