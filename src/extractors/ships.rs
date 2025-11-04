use anyhow::Result;
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;

use crate::models::ship::{ShipMeta, ShipMetaMap};
use crate::parsers::XmlEntry;

/// Extract ship macro metadata: cargo capacity/type and optional speed (if present)
pub fn extract_ships(xml_entries: &[XmlEntry]) -> Result<ShipMetaMap> {
    let mut ships: ShipMetaMap = HashMap::new();

    for entry in xml_entries {
        if let Ok(extracted) = extract_ships_from_xml(&entry.content) {
            for (macro_name, meta) in extracted {
                ships.entry(macro_name).or_insert(meta);
            }
        }
    }

    Ok(ships)
}

fn extract_ships_from_xml(xml: &str) -> Result<ShipMetaMap> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut out: ShipMetaMap = HashMap::new();
    let mut current_macro: Option<String> = None;
    let mut meta: ShipMeta = ShipMeta { cargo_capacity: None, cargo_type: None, max_speed: None };
    let mut depth: usize = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                depth += 1;
                let tag = e.name().as_ref().to_vec();
                if tag.as_slice() == b"macro" {
                    let mut name: Option<String> = None;
                    for a in e.attributes() {
                        if let Ok(a) = a {
                            if a.key.as_ref() == b"name" {
                                name = Some(String::from_utf8_lossy(&a.value).to_string());
                            }
                        }
                    }
                    if let Some(n) = name {
                        if n.starts_with("ship_") {
                            current_macro = Some(n);
                            meta = ShipMeta { cargo_capacity: None, cargo_type: None, max_speed: None };
                        } else {
                            current_macro = None;
                        }
                    }
                } else if current_macro.is_some() {
                    // While inside a ship macro, probe attributes for capacity/type/speed-like keys
                    let mut local_cargo: Option<i32> = None;
                    let mut local_type: Option<String> = None;
                    let mut local_speed: Option<f64> = None;
                    for a in e.attributes() {
                        if let Ok(a) = a {
                            let k = a.key.as_ref();
                            let v = String::from_utf8_lossy(&a.value).to_string();
                            // capacity/size
                            if matches!(k, b"capacity" | b"max" | b"maxcargo") {
                                if let Ok(n) = v.parse::<i32>() { local_cargo = Some(n); }
                            }
                            // cargo type tags
                            if matches!(k, b"tags" | b"cargo" | b"type") {
                                let lv = v.to_lowercase();
                                if lv.contains("container") { local_type = Some("Container".to_string()); }
                                else if lv.contains("solid") { local_type = Some("Solid".to_string()); }
                                else if lv.contains("liquid") { local_type = Some("Liquid".to_string()); }
                                else if lv.contains("energy") { local_type = Some("Energy".to_string()); }
                            }
                            // speed hints
                            if matches!(k, b"maxspeed" | b"speed") {
                                if let Ok(f) = v.parse::<f64>() { local_speed = Some(f); }
                            }
                        }
                    }
                    if let Some(n) = local_cargo { meta.cargo_capacity = Some(meta.cargo_capacity.unwrap_or(0).max(n)); }
                    if let Some(t) = local_type { meta.cargo_type = Some(t); }
                    if let Some(s) = local_speed { meta.max_speed = Some(meta.max_speed.unwrap_or(0.0).max(s)); }
                }
            }
            Ok(Event::Empty(ref e)) => {
                let tag = e.name().as_ref().to_vec();
                if tag.as_slice() == b"macro" {
                    // solitary empty macro: ignore
                } else if current_macro.is_some() {
                    let mut local_cargo: Option<i32> = None;
                    let mut local_type: Option<String> = None;
                    let mut local_speed: Option<f64> = None;
                    for a in e.attributes() {
                        if let Ok(a) = a {
                            let k = a.key.as_ref();
                            let v = String::from_utf8_lossy(&a.value).to_string();
                            if matches!(k, b"capacity" | b"max" | b"maxcargo") {
                                if let Ok(n) = v.parse::<i32>() { local_cargo = Some(n); }
                            }
                            if matches!(k, b"tags" | b"cargo" | b"type") {
                                let lv = v.to_lowercase();
                                if lv.contains("container") { local_type = Some("Container".to_string()); }
                                else if lv.contains("solid") { local_type = Some("Solid".to_string()); }
                                else if lv.contains("liquid") { local_type = Some("Liquid".to_string()); }
                                else if lv.contains("energy") { local_type = Some("Energy".to_string()); }
                            }
                            if matches!(k, b"maxspeed" | b"speed") {
                                if let Ok(f) = v.parse::<f64>() { local_speed = Some(f); }
                            }
                        }
                    }
                    if let Some(n) = local_cargo { meta.cargo_capacity = Some(meta.cargo_capacity.unwrap_or(0).max(n)); }
                    if let Some(t) = local_type { meta.cargo_type = Some(t); }
                    if let Some(s) = local_speed { meta.max_speed = Some(meta.max_speed.unwrap_or(0.0).max(s)); }
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name().as_ref() == b"macro" {
                    if let Some(n) = current_macro.take() {
                        out.insert(n.to_lowercase(), meta.clone());
                    }
                }
                if depth > 0 { depth -= 1; }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(out)
}
