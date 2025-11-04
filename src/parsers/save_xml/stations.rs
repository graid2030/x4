use anyhow::Result;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::collections::{HashMap, HashSet};

use crate::models::{StationCount, StationSummary};
use crate::parsers::game_xml::{clean_name, resolve_name};

#[derive(Debug)]
struct ComponentAttrs {
    class: Option<String>,
    code: Option<String>,
    macro_name: Option<String>,
    owner: Option<String>,
    name_attr: Option<String>,
}

impl ComponentAttrs {
    fn from_bytes(start: &BytesStart<'_>) -> Self {
        let mut attrs = ComponentAttrs {
            class: None,
            code: None,
            macro_name: None,
            owner: None,
            name_attr: None,
        };

        for attr in start.attributes().flatten() {
            match attr.key.as_ref() {
                b"class" => attrs.class = Some(String::from_utf8_lossy(&attr.value).to_string()),
                b"code" => attrs.code = Some(String::from_utf8_lossy(&attr.value).to_string()),
                b"macro" => {
                    attrs.macro_name = Some(String::from_utf8_lossy(&attr.value).to_string())
                }
                b"owner" => attrs.owner = Some(String::from_utf8_lossy(&attr.value).to_string()),
                b"name" => attrs.name_attr = Some(String::from_utf8_lossy(&attr.value).to_string()),
                _ => {}
            }
        }

        attrs
    }
}

fn current_sector(stack: &[Option<String>]) -> Option<String> {
    stack.iter().rev().flatten().next().cloned()
}

fn handle_station(
    attrs: &ComponentAttrs,
    sector_stack: &[Option<String>],
    component_names: &HashMap<String, String>,
    faction_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
    stations: &mut Vec<StationSummary>,
    counts: &mut HashMap<String, StationCount>,
    seen_codes: &mut HashSet<String>,
) {
    if attrs.class.as_deref() != Some("station") {
        return;
    }

    let sector_code = match current_sector(sector_stack) {
        Some(code) => code,
        None => return,
    };

    let code = match attrs.code.clone().or_else(|| attrs.macro_name.clone()) {
        Some(code) => code,
        None => return,
    };

    if !seen_codes.insert(code.clone()) {
        return;
    }

    let mut name = attrs.name_attr.clone().map(|raw| {
        if raw.starts_with('{') {
            resolve_name(&raw, localization)
        } else {
            raw
        }
    });

    if name.as_ref().map(|n| n.trim().is_empty()).unwrap_or(true) {
        if let Some(macro_name) = attrs.macro_name.as_ref() {
            let key = macro_name.to_lowercase();
            if let Some(default_name) = component_names.get(&key) {
                name = Some(default_name.clone());
            }
        }
    }

    let macro_name = attrs.macro_name.clone().unwrap_or_else(|| code.clone());

    let final_name = clean_name(&name.unwrap_or_else(|| code.clone()));
    let owner = attrs.owner.clone();
    let owner_name = owner
        .as_ref()
        .and_then(|o| faction_names.get(o.as_str()).cloned())
        .or_else(|| owner.clone());

    let entry = counts.entry(sector_code.clone()).or_insert((0, 0));
    entry.0 += 1;
    if owner.as_deref() == Some("player") {
        entry.1 += 1;
    }

    stations.push(StationSummary {
        code,
        name: final_name,
        macro_name,
        sector_code,
        owner,
        owner_name,
    });
}

pub struct StationParseResult {
    pub stations: Vec<StationSummary>,
    pub counts: HashMap<String, StationCount>,
}

pub fn extract_station_data(
    xml_content: &str,
    component_names: &HashMap<String, String>,
    faction_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
) -> Result<StationParseResult> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut sector_stack: Vec<Option<String>> = Vec::new();
    let mut stations = Vec::new();
    let mut counts: HashMap<String, StationCount> = HashMap::new();
    let mut seen_codes: HashSet<String> = HashSet::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"component" => {
                let attrs = ComponentAttrs::from_bytes(e);
                handle_station(
                    &attrs,
                    &sector_stack,
                    component_names,
                    faction_names,
                    localization,
                    &mut stations,
                    &mut counts,
                    &mut seen_codes,
                );

                if attrs.class.as_deref() == Some("sector") {
                    sector_stack.push(attrs.code.clone());
                } else {
                    sector_stack.push(None);
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"component" => {
                sector_stack.pop();
            }
            Ok(Event::Empty(ref e)) if e.name().as_ref() == b"component" => {
                let attrs = ComponentAttrs::from_bytes(e);
                handle_station(
                    &attrs,
                    &sector_stack,
                    component_names,
                    faction_names,
                    localization,
                    &mut stations,
                    &mut counts,
                    &mut seen_codes,
                );
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow::anyhow!("XML parse error: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(StationParseResult { stations, counts })
}
