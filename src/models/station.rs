use serde::{Deserialize, Serialize};

// Reserved for future use when implementing detailed station tracking
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub code: String,
    pub name: String,
    pub macro_name: String,
    pub sector_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationSummary {
    pub code: String,
    pub name: String,
    pub macro_name: String,
    pub sector_code: String,
    pub owner: Option<String>,
    pub owner_name: Option<String>,
}

pub type StationCount = (usize, usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationWare {
    pub ware: String,
    pub price: f64,
    pub amount: i32,
    pub trade_type: TradeType,
    pub station_name: String,
    pub station_code: String,
    pub sector_code: String,
    pub sector_name: String,
    pub sector_owner: Option<String>,
}
