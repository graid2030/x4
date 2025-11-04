use axum::http::StatusCode;
use regex::Regex;
use std::collections::HashSet;

use crate::parsers::game_xml::resolve_name;
use crate::parsers::{find_cat_files, CatDatReader};
use crate::services::GameDataCache;

pub fn apply_sector_name_overrides(game_data: &mut GameDataCache) {
    if let Ok(content) = std::fs::read_to_string("x4-names.json") {
        if let Ok(json_map) =
            serde_json::from_str::<std::collections::HashMap<String, String>>(&content)
        {
            for (k, v) in json_map {
                game_data.sector_names.insert(k.to_lowercase(), v);
            }
        }
    }
}

pub fn ensure_sector_name_coverage(
    game_data: &mut GameDataCache,
    game_path: &str,
    lang_id: &str,
    save_content: &str,
    cache_path: &str,
) -> Result<(), StatusCode> {
    let macros_in_save = collect_sector_macros(save_content);
    let missing_count = macros_in_save
        .iter()
        .filter(|m| {
            !game_data.sector_names.contains_key(*m) && !game_data.component_names.contains_key(*m)
        })
        .count();

    if missing_count == 0 {
        return Ok(());
    }

    let sample_missing: Vec<String> = macros_in_save
        .iter()
        .filter(|m| {
            !game_data.sector_names.contains_key(*m) && !game_data.component_names.contains_key(*m)
        })
        .take(5)
        .cloned()
        .collect();

    eprintln!(
        "DIAG REEXTRACT: missing {} macro names from game data (sample: {:?}) — forcing re-extract",
        missing_count, sample_missing
    );

    *game_data = GameDataCache::extract_from_game(game_path, lang_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    apply_sector_name_overrides(game_data);
    let _ = game_data.save_to_file(cache_path);

    let missing_after: Vec<String> = macros_in_save
        .iter()
        .filter(|m| {
            !game_data.sector_names.contains_key(*m) && !game_data.component_names.contains_key(*m)
        })
        .cloned()
        .collect();

    if missing_after.is_empty() {
        return Ok(());
    }

    let patterns: Vec<String> = (1..=20).map(|n| format!("{:02}.cat", n)).collect();
    let pattern_refs: Vec<&str> = patterns.iter().map(|s| s.as_str()).collect();
    let mut all_files =
        find_cat_files(game_path, &pattern_refs).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    all_files.sort();
    all_files.dedup();

    let mut all_xml = Vec::new();
    for cat_file in &all_files {
        let reader = CatDatReader::new(cat_file).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Ok(entries) = reader.extract_xml_files() {
            all_xml.extend(entries);
        }
    }

    let re_tpl_start = r#"(?s)<dataset[^>]*\bmacro\s*=\s*"#;
    let re_tpl_mid = r#""[^>]*>.*?<identification[^>]*\bname\s*=\s*"([^"]+)""#;
    let mut filled = 0usize;

    for macro_id in &missing_after {
        let pattern = format!("{}{}{}", re_tpl_start, regex::escape(macro_id), re_tpl_mid);
        let re = Regex::new(&pattern).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
            game_data.sector_names.insert(macro_id.clone(), name);
            filled += 1;
        }
    }

    eprintln!("DIAG TEXTSCAN: filled {} missing macro names", filled);
    Ok(())
}

fn collect_sector_macros(xml: &str) -> HashSet<String> {
    use quick_xml::{events::Event, Reader};

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut set = HashSet::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                if e.name().as_ref() == b"component" {
                    let mut is_sector = false;
                    let mut macro_name: Option<String> = None;
                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"class" if attr.value.as_ref() == b"sector" => is_sector = true,
                            b"macro" => {
                                macro_name = Some(String::from_utf8_lossy(&attr.value).to_string())
                            }
                            _ => {}
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
