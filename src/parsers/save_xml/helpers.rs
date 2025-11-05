use std::collections::HashMap;
use crate::parsers::game_xml::resolve_name;

#[derive(Debug, Clone)]
pub struct Context {
    pub tag: String,
    pub attrs: HashMap<String, String>,
    #[allow(dead_code)]
    pub depth: usize,
    pub source_entry: Option<String>,
    pub source_nameindex: Option<u32>,
}

/// Find the sector code by walking up the context stack
pub fn find_parent_sector(stack: &[Context]) -> Option<String> {
    for ctx in stack.iter().rev() {
        if ctx.tag == "component" {
            if let Some(class) = ctx.attrs.get("class") {
                if class == "sector" {
                    return ctx.attrs.get("code").cloned();
                }
            }
        }
    }
    None
}

/// Find parent station component and connection in stack
pub fn find_parent_station(stack: &[Context]) -> Option<&Context> {
    // Nearest ancestor component with class="station" (ignore wrecks)
    for ctx in stack.iter().rev() {
        if ctx.tag == "component" {
            if let Some(class) = ctx.attrs.get("class") {
                if class == "station" {
                    if let Some(state) = ctx.attrs.get("state") {
                        if state == "wreck" { return None; }
                    }
                    return Some(ctx);
                }
            }
        }
        // Stop if we walked up to a sector boundary
        if ctx.tag == "component" {
            if let Some(class) = ctx.attrs.get("class") {
                if class == "sector" { return None; }
            }
        }
    }
    None
}

/// Convert number to Roman numerals
fn to_roman(n: u32) -> String {
    let values = [(10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")];
    let mut result = String::new();
    let mut num = n;
    for (val, symbol) in values {
        while num >= val {
            result.push_str(symbol);
            num -= val;
        }
    }
    result
}

/// Capitalize each word
fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Build factory name from source entry
/// entry format: "arg_hullparts" → "ARG Hull Parts Factory I"
fn build_factory_name(entry: &str, nameindex: u32) -> String {
    let parts: Vec<&str> = entry.split('_').collect();
    if parts.len() >= 2 {
        let faction = parts[0].to_uppercase();
        let ware = parts[1..].join(" ");
        let roman = to_roman(nameindex);
        format!("{} {} Factory {}", faction, capitalize_words(&ware), roman)
    } else {
        entry.to_string()
    }
}

/// Get station name from component (replicates Python getStationName logic + factory name support)
pub fn get_station_name(
    station: &Context,
    component_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
) -> String {
    // Priority 1: Custom player name
    let save_name = station.attrs.get("name").map(|name_attr| {
        let resolved = if name_attr.starts_with('{') {
            resolve_name(name_attr, localization)
        } else {
            name_attr.clone()
        };
        crate::parsers::game_xml::clean_name(&resolved)
    });

    if let Some(sn) = save_name.as_ref() {
        if !sn.is_empty() {
            // Check if it's not just the default base name
            let base_name = station
                .attrs
                .get("macro")
                .and_then(|m| component_names.get(&m.to_lowercase()))
                .cloned();

            if let Some(bn) = base_name.as_ref() {
                if sn.to_lowercase() != bn.to_lowercase() {
                    return sn.clone();
                }
            } else {
                return sn.clone();
            }
        }
    }

    // Priority 2: Factory name from source/entry
    if let Some(entry) = &station.source_entry {
        let nameindex = station.source_nameindex
            .or_else(|| station.attrs.get("nameindex").and_then(|s| s.parse().ok()))
            .unwrap_or(1);
        return build_factory_name(entry, nameindex);
    }

    // Priority 3: Base name from component_names
    if let Some(base_name) = station.attrs.get("macro")
        .and_then(|m| component_names.get(&m.to_lowercase()))
    {
        let nameindex = station.attrs.get("nameindex")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        if nameindex > 0 {
            return format!("{} #{}", base_name, nameindex);
        }
        return base_name.clone();
    }

    // Fallback to code or macro
    station
        .attrs
        .get("code")
        .or_else(|| station.attrs.get("macro"))
        .cloned()
        .unwrap_or_else(|| "Station".to_string())
}

/// Get sector name from sector code and macro
pub fn get_sector_name(
    sector_code: &str,
    stack: &[Context],
    sector_names: &HashMap<String, String>,
) -> String {
    // Find the sector component in stack
    for ctx in stack.iter().rev() {
        if ctx.tag == "component" {
            if let Some(class) = ctx.attrs.get("class") {
                if class == "sector" {
                    if let Some(code) = ctx.attrs.get("code") {
                        if code == sector_code {
                            // Try to get name from sector_names by macro
                            if let Some(macro_name) = ctx.attrs.get("macro") {
                                let key = macro_name.to_lowercase();
                                if let Some(name) = sector_names.get(&key) {
                                    return name.clone();
                                }
                            }
                            // Fallback to code
                            return sector_code.to_string();
                        }
                    }
                }
            }
        }
    }
    sector_code.to_string()
}
