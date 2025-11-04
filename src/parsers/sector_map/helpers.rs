use std::collections::HashMap;

/// Simple name resolution for localization
pub fn resolve_name(
    name: &str,
    localization: &HashMap<String, HashMap<String, String>>,
) -> Option<String> {
    if !name.starts_with('{') || !name.ends_with('}') {
        return None;
    }

    let inner = &name[1..name.len() - 1];
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 2 {
        return None;
    }

    let page = parts[0];
    let entry = parts[1];

    localization
        .get(page)
        .and_then(|page_map| page_map.get(entry))
        .cloned()
}

/// Get station name using same logic as in save_xml.rs
pub fn get_station_name(
    attrs: &HashMap<String, String>,
    component_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
) -> String {
    if let Some(macro_name) = attrs.get("macro") {
        let key = macro_name.to_lowercase();
        if let Some(name) = component_names.get(&key) {
            return name.clone();
        }
    }

    if let Some(name_attr) = attrs.get("name") {
        if name_attr.starts_with('{') {
            if let Some(resolved) = resolve_name(name_attr, localization) {
                return resolved;
            }
        }
    }

    attrs
        .get("code")
        .or_else(|| attrs.get("macro"))
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string())
}
