use anyhow::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

use crate::parsers::{game_xml, XmlEntry};

/// Extract sector names from dataset XML files
pub fn extract_sector_names(
    xml_entries: &[XmlEntry],
    localization: &HashMap<String, HashMap<String, String>>,
) -> Result<HashMap<String, String>> {
    let mut sector_names = HashMap::new();

    for entry in xml_entries {
        let mut reader = Reader::from_str(&entry.content);
        reader.trim_text(true);
        let mut buf = Vec::new();

        let mut current_macro: Option<String> = None;
        let mut in_properties = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    match e.name().as_ref() {
                        b"dataset" => {
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key.as_ref() == b"macro" {
                                        current_macro =
                                            Some(String::from_utf8_lossy(&attr.value).to_string());
                                        in_properties = false;
                                    }
                                }
                            }
                        }
                        b"properties" => {
                            if current_macro.is_some() {
                                in_properties = true;
                            }
                        }
                        b"identification" => {
                            // Only process identification inside properties tag (matching Python XPath: .//properties/identification)
                            if let Some(macro_name) = &current_macro {
                                if in_properties {
                                    for attr in e.attributes() {
                                        if let Ok(attr) = attr {
                                            if attr.key.as_ref() == b"name" {
                                                let name_ref =
                                                    String::from_utf8_lossy(&attr.value).to_string();
                                                let mut resolved =
                                                    game_xml::resolve_name(&name_ref, localization);
                                                resolved = game_xml::clean_name(&resolved);
                                                sector_names.insert(macro_name.to_lowercase(), resolved);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    match e.name().as_ref() {
                        b"dataset" => {
                            current_macro = None;
                            in_properties = false;
                        }
                        b"properties" => {
                            in_properties = false;
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }
    }

    Ok(sector_names)
}

/// Build a mapping from sector code (e.g., "AAM-257") to resolved name (e.g., "Family Zhin").
pub fn extract_sector_code_names(
    xml_entries: &[XmlEntry],
    sector_names: &HashMap<String, String>,
    component_names: &HashMap<String, String>,
) -> Result<HashMap<String, String>> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut map = HashMap::new();

    // We specifically mirror the Python path:
    // .//universe/component/connections/connection/component/connections/connection/component[@class='sector']
    // We implement this by tracking the current element stack and matching the suffix when we hit a component.

    for entry in xml_entries {
        let mut reader = Reader::from_str(&entry.content);
        reader.trim_text(true);
        let mut buf = Vec::new();
        let mut stack: Vec<String> = Vec::new();

        // Helper to check if current stack ends with the desired path ending with a component
        let is_target_sector_position = |stack: &Vec<String>| -> bool {
            // Expected suffix (without the final component which we’re at already):
            // universe, component, connections, connection, component, connections, connection
            let suffix = [
                "universe",
                "component",
                "connections",
                "connection",
                "component",
                "connections",
                "connection",
            ];
            if stack.len() < suffix.len() { return false; }
            let start = stack.len() - suffix.len();
            for (i, seg) in suffix.iter().enumerate() {
                if stack[start + i] != *seg { return false; }
            }
            true
        };

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    stack.push(tag.clone());

                    if tag == "component" && is_target_sector_position(&stack) {
                        let mut class = None;
                        let mut code = None;
                        let mut macro_name = None;
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                match attr.key.as_ref() {
                                    b"class" => class = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                    b"code" => code = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                    b"macro" => macro_name = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                    _ => {}
                                }
                            }
                        }
                        if class.as_deref() == Some("sector") {
                            if let (Some(c), Some(m)) = (code, macro_name) {
                                let key = m.to_lowercase();
                                let resolved = sector_names
                                    .get(&key)
                                    .or_else(|| component_names.get(&key))
                                    .cloned()
                                    .unwrap_or_else(|| m.clone());
                                map.entry(c).or_insert(resolved);
                            }
                        }
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();

                    if tag == "component" && is_target_sector_position(&stack) {
                        let mut class = None;
                        let mut code = None;
                        let mut macro_name = None;
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                match attr.key.as_ref() {
                                    b"class" => class = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                    b"code" => code = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                    b"macro" => macro_name = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                    _ => {}
                                }
                            }
                        }
                        if class.as_deref() == Some("sector") {
                            if let (Some(c), Some(m)) = (code, macro_name) {
                                let key = m.to_lowercase();
                                let resolved = sector_names
                                    .get(&key)
                                    .or_else(|| component_names.get(&key))
                                    .cloned()
                                    .unwrap_or_else(|| m.clone());
                                map.entry(c).or_insert(resolved);
                            }
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    if let Some(last) = stack.pop() {
                        let _ = last; // keep stack balanced regardless
                    }
                    let _ = tag;
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }
    }

    Ok(map)
}
