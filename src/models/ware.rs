use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportType {
    Container,
    Solid,
    Liquid,
    Energy,
}

impl Default for TransportType {
    fn default() -> Self {
        TransportType::Container
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WareMeta {
    pub id: String,
    pub transport: TransportType,
    pub volume: Option<f64>,
    // Raw localization reference for display name (e.g., "{20104,10001}")
    pub name_ref: Option<String>,
}

pub type WareMetaMap = HashMap<String, WareMeta>;
