use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{Sector, PilotInfo};
use crate::services::GameDataCache;

#[derive(Clone)]
pub struct AppState {
    pub game_data: Arc<RwLock<Option<GameDataCache>>>,
    pub save_data: Arc<RwLock<Option<SaveData>>>,
}

#[derive(Clone)]
pub struct SaveData {
    pub sectors: Vec<Sector>,
    pub save_path: String,
    pub pilot: Option<PilotInfo>,
}
