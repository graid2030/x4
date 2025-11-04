use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::parsers::game_xml::resolve_name;
use crate::parsers::{extract_sectors, find_cat_files, load_save_file, CatDatReader};
use crate::services::{GameDataCache, SavedPaths};

use super::common::{AppState, SaveData};

#[derive(Deserialize)]
pub struct InitRequest {
    pub game_path: String,
    pub saves_dir: String,
    pub selected_save: String,
    #[serde(default = "default_lang_id")]
    pub lang_id: String,
}

fn default_lang_id() -> String {
    "44".to_string()
}

#[derive(Serialize)]
pub struct InitResponse {
    pub success: bool,
    pub message: String,
}

/// Initialize the system with game and save paths
pub async fn init_handler(
    State(state): State<AppState>,
    Json(req): Json<InitRequest>,
) -> Result<Json<InitResponse>, StatusCode> {
    // Load or extract game data
    let cache_path = "x4-cache.json".to_string();
    let mut game_data = GameDataCache::load_or_extract(&req.game_path, &cache_path, &req.lang_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Optional: override/augment sector name map with x4-names.json (macro -> clean name)
    if let Ok(content) = std::fs::read_to_string("x4-names.json") {
        if let Ok(json_map) =
            serde_json::from_str::<std::collections::HashMap<String, String>>(&content)
        {
            for (k, v) in json_map {
                game_data.sector_names.insert(k.to_lowercase(), v);
            }
        }
    }

    // Build full save file path
    let save_path = format!("{}\\{}", req.saves_dir, req.selected_save);

    // Load save file
    let save_content = load_save_file(&save_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // DIAG: localization presence (even when loaded from cache)
    let has_20004 = game_data.localization.contains_key("20004");
    let page20004_len = game_data
        .localization
        .get("20004")
        .map(|p| p.len())
        .unwrap_or(0);
    let sample_keys: Vec<String> = game_data
        .localization
        .get("20004")
        .map(|p| p.keys().take(3).cloned().collect())
        .unwrap_or_else(|| Vec::new());
    eprintln!(
        "DIAG LOC (cache/load): pages={} has20004={} page20004_entries={} sample={:?}",
        game_data.localization.len(),
        has_20004,
        page20004_len,
        sample_keys
    );

    // Quick scan: collect sector macros from save and ensure name maps cover them; if not, force re-extract
    let macros_in_save = collect_sector_macros(&save_content);
    let missing_macros: Vec<String> = macros_in_save
        .iter()
        .filter(|m| {
            !game_data.sector_names.contains_key(*m) && !game_data.component_names.contains_key(*m)
        })
        .take(5)
        .cloned()
        .collect();
    let missing_count = macros_in_save
        .iter()
        .filter(|m| {
            !game_data.sector_names.contains_key(*m) && !game_data.component_names.contains_key(*m)
        })
        .count();
    if missing_count > 0 {
        eprintln!(
            "DIAG REEXTRACT: missing {} macro names from game data (sample: {:?}) — forcing re-extract",
            missing_count,
            missing_macros
        );
        game_data = GameDataCache::extract_from_game(&req.game_path, &req.lang_id)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        // Re-apply optional override
        if let Ok(content) = std::fs::read_to_string("x4-names.json") {
            if let Ok(json_map) =
                serde_json::from_str::<std::collections::HashMap<String, String>>(&content)
            {
                for (k, v) in json_map {
                    game_data.sector_names.insert(k.to_lowercase(), v);
                }
            }
        }
        // Save refreshed cache
        let _ = game_data.save_to_file(&cache_path);

        // Recompute missing after re-extract
        let missing_after: Vec<String> = macros_in_save
            .iter()
            .filter(|m| {
                !game_data.sector_names.contains_key(*m)
                    && !game_data.component_names.contains_key(*m)
            })
            .cloned()
            .collect();
        if !missing_after.is_empty() {
            // Last-resort textual scan in CAT XMLs for those specific macros
            let patterns: Vec<String> = (1..=20).map(|n| format!("{:02}.cat", n)).collect();
            let pattern_refs: Vec<&str> = patterns.iter().map(|s| s.as_str()).collect();
            let mut all_files = find_cat_files(&req.game_path, &pattern_refs)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            all_files.sort();
            all_files.dedup();
            let mut all_xml = Vec::new();
            for cat_file in &all_files {
                let reader =
                    CatDatReader::new(cat_file).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                if let Ok(entries) = reader.extract_xml_files() {
                    all_xml.extend(entries);
                }
            }

            let mut filled = 0usize;
            let _missing_set: std::collections::HashSet<String> =
                missing_after.iter().cloned().collect();
            let re_tpl_start = r#"(?s)<dataset[^>]*\bmacro\s*=\s*"#;
            let re_tpl_mid = r#""[^>]*>.*?<identification[^>]*\bname\s*=\s*"([^"]+)""#;
            for m in &missing_after {
                let pat = format!("{}{}{}", re_tpl_start, regex::escape(m), re_tpl_mid);
                let re = regex::Regex::new(&pat).unwrap();
                let mut found: Option<String> = None;
                for entry in &all_xml {
                    if let Some(cap) = re.captures(&entry.content) {
                        let raw = cap.get(1).map(|g| g.as_str()).unwrap_or("");
                        let resolved = resolve_name(raw, &game_data.localization);
                        found = Some(resolved);
                        break;
                    }
                }
                if let Some(name) = found {
                    game_data.sector_names.insert(m.clone(), name);
                    filled += 1;
                }
            }
            eprintln!("DIAG TEXTSCAN: filled {} missing macro names", filled);
        }
    }

    let sectors = extract_sectors(
        &save_content,
        &game_data.sector_names,
        &game_data.component_names,
        &game_data.sector_code_names,
        &game_data.localization,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = sectors.len();

    // Extract pilot info
    let pilot =
        crate::parsers::pilot_xml::extract_pilot_info(&save_content, &game_data.localization).ok();

    // Persist last-used paths into cache file
    game_data.last_paths = Some(SavedPaths {
        game_path: req.game_path.clone(),
        saves_dir: req.saves_dir.clone(),
    });
    let _ = game_data.save_to_file(&cache_path);

    // Store in state
    *state.game_data.write().await = Some(game_data);
    *state.save_data.write().await = Some(SaveData {
        sectors,
        save_path,
        pilot,
        last_modified: None,
        content_hash: None,
        trades_by_sector: HashMap::new(),
        station_counts: HashMap::new(),
        stations: Vec::new(),
        station_lookup: HashMap::new(),
    });

    state.save_repository.ensure_latest().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(InitResponse {
        success: true,
        message: "Initialized successfully".to_string(),
    }))
}

fn collect_sector_macros(xml: &str) -> std::collections::HashSet<String> {
    use quick_xml::{events::Event, Reader};
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut set = std::collections::HashSet::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                if e.name().as_ref() == b"component" {
                    let mut is_sector = false;
                    let mut macro_name: Option<String> = None;
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"class" if attr.value.as_ref() == b"sector" => is_sector = true,
                                b"macro" => {
                                    macro_name =
                                        Some(String::from_utf8_lossy(&attr.value).to_string())
                                }
                                _ => {}
                            }
                        }
                    }
                    if is_sector {
                        if let Some(m) = macro_name {
                            set.insert(m.to_lowercase());
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    set
}
