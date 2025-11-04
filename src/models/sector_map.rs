use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapStation {
    pub name: String,
    pub code: String,
    pub owner: Option<String>,
    pub position: Position,
    pub macro_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapShip {
    pub name: String,
    pub code: String,
    pub owner: Option<String>,
    pub position: Position,
    pub class: String,
    pub macro_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapGate {
    pub name: String,
    pub code: String,
    pub position: Position,
    pub destination: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapAsteroid {
    pub code: String,
    pub position: Position,
    pub macro_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapResource {
    pub name: String,
    pub code: String,
    pub position: Position,
    pub resource_type: String,
    pub macro_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorMapData {
    pub sector_code: String,
    pub sector_name: String,
    pub owner: Option<String>,
    pub stations: Vec<MapStation>,
    pub ships: Vec<MapShip>,
    pub gates: Vec<MapGate>,
    pub asteroids: Vec<MapAsteroid>,
    pub resources: Vec<MapResource>,
}
