use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{PilotInfo, Sector};
use crate::services::GameDataRepository;

#[derive(Clone)]
pub struct AppState {
    pub game_data: Arc<RwLock<Option<GameDataRepository>>>,
    pub save_data: Arc<RwLock<Option<SaveData>>>,
}

#[derive(Clone)]
pub struct SaveData {
    pub sectors: Vec<Sector>,
    pub save_path: String,
    pub pilot: Option<PilotInfo>,
}
