use std::collections::HashMap;
use crate::parsers::game_xml::{clean_name, resolve_name};

pub fn text_attr(attrs: &HashMap<String, String>, key: &str) -> Option<String> {
    attrs.get(key).cloned()
}

pub fn has_owner_player(attrs: &HashMap<String, String>) -> bool {
    attrs.get("owner").map(|v| v.as_str()) == Some("player")
}

pub fn is_station_or_ship(class: &str) -> bool {
    class == "station" || class.contains("ship")
}

pub fn resolve_component_name(
    attrs: &HashMap<String, String>,
    component_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
) -> String {
    if let Some(name_ref) = attrs.get("name") {
        return clean_name(&resolve_name(name_ref, localization));
    }
    if let Some(macro_name) = attrs.get("macro") {
        let key = macro_name.to_lowercase();
        if let Some(base) = component_names.get(&key) {
            let nameindex = attrs.get("nameindex").map(|s| s.as_str()).unwrap_or("0");
            if nameindex != "0" {
                return format!("{} #{}", base, nameindex);
            }
            return base.clone();
        }
    }
    attrs.get("code").cloned().unwrap_or_else(|| "Unnamed".to_string())
}

pub fn get_sector_from_stack(
    stack: &[crate::parsers::assets::types::Ctx],
    sector_names: &HashMap<String, String>,
) -> (Option<String>, Option<String>) {
    for ctx in stack.iter().rev() {
        if ctx.tag == "component" {
            if let Some(class) = ctx.attrs.get("class") {
                if class == "sector" {
                    let code = ctx.attrs.get("code").cloned();
                    let name = ctx
                        .attrs
                        .get("macro")
                        .and_then(|m| sector_names.get(&m.to_lowercase()))
                        .cloned();
                    return (code, name);
                }
            }
        }
    }
    (None, None)
}

pub fn derive_ship_meta(
    macro_name: Option<&str>,
    _thruster: Option<&str>,
) -> (Option<i32>, Option<String>, Option<String>, Option<f64>) {
    // Heuristic: parse macro like ship_arg_m_trans_container_01_a_macro
    if let Some(m) = macro_name {
        let lower = m.to_lowercase();
        let mut size: Option<&str> = None;
        if lower.contains("_s_") {
            size = Some("S");
        } else if lower.contains("_m_") {
            size = Some("M");
        } else if lower.contains("_l_") {
            size = Some("L");
        } else if lower.contains("_xl_") {
            size = Some("XL");
        }

        let role = if lower.contains("scout") {
            Some("Scout")
        } else if lower.contains("trans_container") {
            Some("Transport (Container)")
        } else if lower.contains("trans_solid") {
            Some("Transport (Solid)")
        } else if lower.contains("trans_liquid") {
            Some("Transport (Liquid)")
        } else if lower.contains("miner_solid") {
            Some("Miner (Solid)")
        } else if lower.contains("miner_liquid") {
            Some("Miner (Liquid)")
        } else if lower.contains("fighter") {
            Some("Fighter")
        } else if lower.contains("bomber") {
            Some("Bomber")
        } else if lower.contains("carrier") {
            Some("Carrier")
        } else if lower.contains("destroyer") {
            Some("Destroyer")
        } else {
            None
        };

        let ship_type = match (size, role) {
            (Some(s), Some(r)) => Some(format!("{} {}", s, r)),
            (Some(s), None) => Some(s.to_string()),
            _ => None,
        };

        // Without game macro parsing, we can't know numeric capacity/speed -> keep None
        return (None, role.map(|r| r.to_string()), ship_type, None);
    }
    (None, None, None, None)
}
