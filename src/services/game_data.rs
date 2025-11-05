use std::collections::HashMap;

use crate::models::{ship::ShipMetaMap, WareMetaMap};
use crate::parsers::game_xml::resolve_name;

use super::GameDataCache;

/// Repository facade that exposes convenient accessors over the raw
/// [`GameDataCache`].
///
/// Handlers previously reached into the cache's internal maps directly which
/// led to duplicated localization/name resolution code. This wrapper provides
/// focused helpers so the rest of the application can stay lean.
#[derive(Debug, Clone)]
pub struct GameDataRepository {
    cache: GameDataCache,
}

impl GameDataRepository {
    /// Wrap an extracted [`GameDataCache`].
    pub fn new(cache: GameDataCache) -> Self {
        Self { cache }
    }

    /// Map of sector macro → resolved sector name.
    pub fn sector_names_map(&self) -> &HashMap<String, String> {
        &self.cache.sector_names
    }

    /// Map of component macro → resolved component name.
    pub fn component_names_map(&self) -> &HashMap<String, String> {
        &self.cache.component_names
    }

    /// Localization tables keyed by page id.
    pub fn localization_map(&self) -> &HashMap<String, HashMap<String, String>> {
        &self.cache.localization
    }

    /// Metadata for all wares, keyed by ware id.
    pub fn wares(&self) -> &WareMetaMap {
        &self.cache.wares
    }

    /// Metadata for ships, keyed by macro id.
    pub fn ship_meta(&self) -> &ShipMetaMap {
        &self.cache.ship_meta
    }

    /// Map of faction id → localized display name.
    pub fn faction_names_map(&self) -> &HashMap<String, String> {
        &self.cache.faction_names
    }

    /// Resolve a localized sector name for the given code.
    ///
    /// The method checks the sector-code map first, then falls back to the
    /// macro based lookup tables. If nothing matches, the original code is
    /// returned.
    pub fn get_sector_name(&self, code: &str) -> String {
        self.cache
            .sector_code_names
            .get(code)
            .cloned()
            .or_else(|| self.cache.sector_names.get(&code.to_lowercase()).cloned())
            .or_else(|| self.cache.component_names.get(&code.to_lowercase()).cloned())
            .unwrap_or_else(|| code.to_string())
    }

    /// Resolve a station/component macro to a human friendly name.
    pub fn get_station_name(&self, macro_name: &str) -> String {
        self.cache
            .component_names
            .get(&macro_name.to_lowercase())
            .cloned()
            .unwrap_or_else(|| macro_name.to_string())
    }

    /// Resolve the display name for the given ware id.
    pub fn get_ware_display_name(&self, ware_id: &str) -> String {
        self.cache
            .wares
            .get(ware_id)
            .and_then(|meta| meta.name_ref.as_ref())
            .map(|name_ref| resolve_name(name_ref, &self.cache.localization))
            .unwrap_or_else(|| ware_id.to_string())
    }

    /// Resolve the display name for a faction id.
    pub fn get_faction_display_name(&self, faction_id: &str) -> String {
        self.cache
            .faction_names
            .get(faction_id)
            .cloned()
            .unwrap_or_else(|| faction_id.to_string())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ship::ShipMeta, TransportType, WareMeta};

    fn sample_cache() -> GameDataCache {
        let mut wares = WareMetaMap::new();
        wares.insert(
            "ware.energy".to_string(),
            WareMeta {
                id: "ware.energy".to_string(),
                transport: TransportType::Energy,
                volume: Some(1.0),
                name_ref: Some("{100,1}".to_string()),
            },
        );

        let mut localization = HashMap::new();
        localization.insert(
            "100".to_string(),
            HashMap::from([("1".to_string(), "Energy Cells".to_string())]),
        );

        let mut sector_names = HashMap::new();
        sector_names.insert("cluster_macro".to_string(), "Cluster".to_string());

        let mut sector_code_names = HashMap::new();
        sector_code_names.insert("AAM-001".to_string(), "Cluster".to_string());

        let mut component_names = HashMap::new();
        component_names.insert("station_macro".to_string(), "Alpha Station".to_string());

        let mut ship_meta = ShipMetaMap::new();
        ship_meta.insert(
            "ship_macro".to_string(),
            ShipMeta {
                cargo_capacity: Some(100),
                cargo_type: Some("container".to_string()),
                max_speed: Some(300.0),
            },
        );

        let faction_names = HashMap::from([("argon".to_string(), "Argon Federation".to_string())]);

        GameDataCache {
            version: 0,
            wares,
            sector_names,
            component_names,
            sector_code_names,
            localization,
            last_paths: None,
            ship_meta,
            faction_names,
        }
    }

    #[test]
    fn resolves_sector_name_with_fallback() {
        let repo = GameDataRepository::new(sample_cache());
        assert_eq!(repo.get_sector_name("AAM-001"), "Cluster");
        assert_eq!(repo.get_sector_name("UNKNOWN"), "UNKNOWN");
    }

    #[test]
    fn resolves_station_name() {
        let repo = GameDataRepository::new(sample_cache());
        assert_eq!(repo.get_station_name("station_macro"), "Alpha Station");
        assert_eq!(repo.get_station_name("missing"), "missing");
    }

    #[test]
    fn resolves_ware_and_faction_display_names() {
        let repo = GameDataRepository::new(sample_cache());
        assert_eq!(repo.get_ware_display_name("ware.energy"), "Energy Cells");
        assert_eq!(repo.get_ware_display_name("unknown"), "unknown");
        assert_eq!(repo.get_faction_display_name("argon"), "Argon Federation");
        assert_eq!(repo.get_faction_display_name("pirate"), "pirate");
    }

    #[test]
    fn exposes_underlying_maps() {
        let repo = GameDataRepository::new(sample_cache());
        assert!(repo.wares().contains_key("ware.energy"));
        assert!(repo.localization_map().contains_key("100"));
        assert!(repo.ship_meta().contains_key("ship_macro"));
        assert!(repo.component_names_map().contains_key("station_macro"));
    }
}
