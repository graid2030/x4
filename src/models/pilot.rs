use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PilotInfo {
    pub name: String,
    pub credits: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
