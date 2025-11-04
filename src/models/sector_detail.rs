use serde::{Deserialize, Serialize};

/// Station information for sector detail view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationInfo {
    pub code: String,
    pub name: String,
    pub owner: Option<String>,
    pub owner_name: Option<String>,
    pub macro_name: String,
    pub position: (f64, f64, f64), // x, y, z
}

/// Trade offer with station info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOfferDetail {
    pub ware_id: String,
    pub ware_name: String,
    pub trade_type: String, // "buy" or "sell"
    pub price: f64,
    pub amount: i32,
    pub station_code: String,
    pub station_name: String,
}

/// Sector detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorDetailResponse {
    pub code: String,
    pub name: String,
    pub macro_name: String,
    pub owner: Option<String>,
    pub owner_name: Option<String>,
    pub contested: bool,
    pub discovered: bool,
    pub stations: Vec<StationInfo>,
    pub trade_offers: Vec<TradeOfferDetail>,
    pub stats: SectorDetailStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorDetailStats {
    pub total_stations: usize,
    pub player_stations: usize,
    pub buy_offers: usize,
    pub sell_offers: usize,
    pub unique_wares: usize,
}
