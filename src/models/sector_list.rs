use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorListItem {
    pub code: String,
    pub name: String,
    pub macro_name: String,
    pub owner: Option<String>,
    pub owner_name: Option<String>,
    pub contested: bool,
    pub station_count: usize,
    pub player_stations: usize,
    pub buy_offers: usize,
    pub sell_offers: usize,
    pub total_trades: usize,
    pub resource_types: Vec<String>,
    pub discovered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorStats {
    pub total_sectors: usize,
    pub player_owned: usize,
    pub discovered: usize,
    pub total_stations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorsListResponse {
    pub sectors: Vec<SectorListItem>,
    pub stats: SectorStats,
}
