use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub ware: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ware_name: Option<String>,
    pub buy_price: f64,
    pub buy_station: String,
    pub buy_station_code: String,
    pub buy_sector: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_sector_owner: Option<String>,
    pub sell_price: f64,
    pub sell_station: String,
    pub sell_station_code: String,
    pub sell_sector: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_sector_owner: Option<String>,
    pub qty: i32,
    pub unit_profit: f64,
    pub total_profit: f64,
    pub percent_profit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fits: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_qty: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_total_profit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_total_volume: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFilters {
    pub sectors: Option<Vec<String>>,
    pub wares: Option<Vec<String>>,
    pub ship_id: Option<String>,
    pub cargo_volume: Option<f64>,
    pub group_by_ware: bool,
    #[serde(default)]
    pub same_sector_only: bool,
}
