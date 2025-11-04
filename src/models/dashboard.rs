use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardResponse {
    pub player: DashboardPlayer,
    pub stats: DashboardStats,
    pub top_routes: Vec<DashboardRoute>,
    pub sector_summary: Vec<DashboardSector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardPlayer {
    pub name: String,
    pub money: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub game_time: i64, // seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub sectors: usize,
    pub stations: usize,
    pub ships: usize,
    pub npcs: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardRoute {
    pub ware: String,
    pub ware_name: String,
    pub buy_station: String,
    pub buy_sector: String,
    pub sell_station: String,
    pub sell_sector: String,
    pub profit: f64,
    pub buy_price: f64,
    pub sell_price: f64,
    pub amount: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSector {
    pub code: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    pub station_count: usize,
    pub player_stations: usize,
}
