use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAsset {
    pub id: String,
    pub class: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macro_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerNpc {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub piloting: i32,
    pub engineering: i32,
    pub boarding: i32,
    pub management: i32,
    pub morale: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPropertyResponse {
    pub assets: Vec<PlayerAsset>,
    pub npcs: Vec<PlayerNpc>,
}
