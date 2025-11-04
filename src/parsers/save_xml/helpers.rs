use std::collections::HashMap;
use crate::parsers::game_xml::resolve_name;

#[derive(Debug, Clone)]
pub struct Context {
    pub tag: String,
    pub attrs: HashMap<String, String>,
    #[allow(dead_code)]
    pub depth: usize,
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

/// Get station name from component (replicates Python getStationName logic)
pub fn get_station_name(
    station: &Context,
    component_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
) -> String {
    // Base name from meta/component_names by macro
    let base_name = station
        .attrs
        .get("macro")
        .and_then(|m| component_names.get(&m.to_lowercase()))
        .cloned();

    // Save-provided name (may be more specific), resolved + cleaned
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
            if let Some(bn) = base_name.as_ref() {
                if sn.to_lowercase() != bn.to_lowercase() {
                    return sn.clone();
                }
            } else {
                return sn.clone();
            }
        }
    }

    if let Some(bn) = base_name {
        let nameindex = station
            .attrs
            .get("nameindex")
            .map(|s| s.as_str())
            .unwrap_or("0");
        if nameindex != "0" {
            return format!("{} #{}", bn, nameindex);
        }
        return bn;
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
