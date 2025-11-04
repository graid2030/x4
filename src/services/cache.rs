use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::extractors::{extract_all_localization, extract_component_names, extract_sector_names, extract_wares};
use crate::extractors::ships::extract_ships;
use crate::models::{WareMetaMap};
use crate::models::ship::ShipMetaMap;
use crate::parsers::{find_cat_files, CatDatReader};

const CURRENT_VERSION: u32 = 10; // Added faction_names extraction

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataCache {
    pub version: u32,
    pub wares: WareMetaMap,
    pub sector_names: HashMap<String, String>,
    pub component_names: HashMap<String, String>,
    #[serde(default)]
    pub sector_code_names: HashMap<String, String>,
    pub localization: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub last_paths: Option<SavedPaths>,
    #[serde(default)]
    pub ship_meta: ShipMetaMap,
    #[serde(default)]
    pub faction_names: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPaths {
    pub game_path: String,
    pub saves_dir: String,
}

impl GameDataCache {
    /// Load from cache file if exists, otherwise extract from game data
    pub fn load_or_extract<P: AsRef<Path>>(
        x4_folder: P,
        cache_path: P,
        lang_id: &str,
    ) -> Result<Self> {
        // Try to load from cache first (and validate version)
        if let Ok(cache) = Self::load_from_file(&cache_path) {
            if cache.version == CURRENT_VERSION {
                println!("Loaded game data from cache (v{})", cache.version);
                return Ok(cache);
            } else {
                let _ = (cache.version, CURRENT_VERSION);
            }
        }

        println!("Extracting game data...");
        let cache = Self::extract_from_game(x4_folder, lang_id)?;

        // Save to cache
        let _ = cache.save_to_file(&cache_path);

        Ok(cache)
    }

    /// Extract all game data from X4 installation
    pub fn extract_from_game<P: AsRef<Path>>(x4_folder: P, lang_id: &str) -> Result<Self> {
        let x4_folder = x4_folder.as_ref();

        // Find relevant CAT files across base + extensions (broader range to include DLC)
        let patterns: Vec<String> = (1..=20).map(|n| format!("{:02}.cat", n)).collect();
        let pattern_refs: Vec<&str> = patterns.iter().map(|s| s.as_str()).collect();
        let mut all_files = find_cat_files(x4_folder, &pattern_refs)?;

        // Remove duplicates
        all_files.sort();
        all_files.dedup();

        // Extract XML from all CAT files
        let mut all_xml = Vec::new();
        for cat_file in &all_files {
            println!("Processing: {}", cat_file);
            let reader = CatDatReader::new(cat_file)?;
            let entries = reader.extract_xml_files()?;
            all_xml.extend(entries);
        }

        println!("Extracting localization...");
        let mut localization = extract_all_localization(&all_xml, lang_id)?;
        let mut chosen_lang = lang_id.to_string();
        // Fallback: if chosen language yields no pages (or misses common sector page 20004),
        // try a small set of common language IDs to ensure sector names resolve.
        if localization.is_empty() || !localization.contains_key("20004") {
            let fallbacks = ["44", "34", "49", "33", "39"]; // EN, RU, DE, FR, IT
            for fb in fallbacks.iter().filter(|l| **l != lang_id) {
                let loc2 = extract_all_localization(&all_xml, fb)?;
                if !loc2.is_empty() {
                    localization = loc2;
                    chosen_lang = fb.to_string();
                    break;
                }
            }
        }
        // Diagnostics: confirm localization presence and common page 20004
        let has_20004 = localization.contains_key("20004");
        let page20004_len = localization.get("20004").map(|p| p.len()).unwrap_or(0);
        let sample_keys: Vec<String> = localization
            .get("20004")
            .map(|p| p.keys().take(3).cloned().collect())
            .unwrap_or_else(|| Vec::new());
        eprintln!(
            "DIAG LOC: chosen_lang={} pages={} has20004={} page20004_entries={} sample={:?}",
            chosen_lang,
            localization.len(),
            has_20004,
            page20004_len,
            sample_keys
        );

        println!("Extracting component names...");
        let component_names = extract_component_names(&all_xml, &localization)?;

        println!("Extracting sector names...");
        let sector_names = extract_sector_names(&all_xml, &localization)?;

        println!("Extracting faction names...");
        let faction_names = crate::extractors::factions::extract_faction_names(&all_xml, &localization)?;

        println!("Extracting ware metadata...");
        let wares = extract_wares(&all_xml)?;

        println!("Extracting ship metadata...");
        let ship_meta = extract_ships(&all_xml)?;

        // Build sector code -> name map using game universe components
        let sector_code_names = crate::extractors::sectors::extract_sector_code_names(
            &all_xml,
            &sector_names,
            &component_names,
        )?;
        let _ = (sector_names.len(), component_names.len(), sector_code_names.len());
        // Targeted diagnostic for AAM-257 macro mapping presence
        let key = "cluster_401_sector001_macro".to_string();
        eprintln!(
            "DIAG MAPS: sector_names.has[{}]={} component_names.has[{}]={} code_map.has[AAM-257]={}",
            key,
            sector_names.contains_key(&key),
            key,
            component_names.contains_key(&key),
            sector_code_names.contains_key("AAM-257")
        );

        let cache = Self {
            version: CURRENT_VERSION,
            wares,
            sector_names,
            component_names,
            sector_code_names,
            localization,
            last_paths: None,
            ship_meta,
            faction_names,
        };

        Ok(cache)
    }

    /// Load cache from JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let cache = serde_json::from_str(&content)?;
        Ok(cache)
    }

    /// Save cache to JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

// (Removed compatibility JSON writers and debug code)
