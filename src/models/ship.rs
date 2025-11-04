use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Reserved for future use when implementing player ship detection
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerShip {
    pub id: String,
    pub name: String,
    pub macro_name: String,
    pub cargo_capacity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speed: Option<f64>,
}

pub type ShipMetaMap = HashMap<String, ShipMeta>;
