use anyhow::Result;
use std::collections::HashMap;

use crate::parsers::{game_xml, XmlEntry};

/// Extract all localization data from XML entries
pub fn extract_all_localization(
    xml_entries: &[XmlEntry],
    lang_id: &str,
) -> Result<HashMap<String, HashMap<String, String>>> {
    let mut localization = HashMap::new();

    for entry in xml_entries {
        if let Ok(loc) = game_xml::extract_localization(&entry.content, lang_id) {
            for (page_id, page_data) in loc {
                localization
                    .entry(page_id)
                    .or_insert_with(HashMap::new)
                    .extend(page_data);
            }
        }
    }

    Ok(localization)
}

/// Extract component names (stations, ships, etc.)
pub fn extract_component_names(
    xml_entries: &[XmlEntry],
    localization: &HashMap<String, HashMap<String, String>>,
) -> Result<HashMap<String, String>> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut component_names = HashMap::new();

    for entry in xml_entries {
        if !entry.name.contains("macros") {
            continue;
        }

        let mut reader = Reader::from_str(&entry.content);
        reader.trim_text(true);
        let mut buf = Vec::new();

        let mut current_macro_name: Option<String> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    match e.name().as_ref() {
                        b"macro" => {
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key.as_ref() == b"name" {
                                        current_macro_name =
                                            Some(String::from_utf8_lossy(&attr.value).to_string());
                                    }
                                }
                            }
                        }
                        b"identification" => {
                            if let Some(macro_name) = &current_macro_name {
                                for attr in e.attributes() {
                                    if let Ok(attr) = attr {
                                        if attr.key.as_ref() == b"name" {
                                            let name_ref =
                                                String::from_utf8_lossy(&attr.value).to_string();
                                            let mut resolved =
                                                game_xml::resolve_name(&name_ref, localization);
                                            resolved = game_xml::clean_name(&resolved);
                                            component_names
                                                .insert(macro_name.to_lowercase(), resolved);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"macro" {
                        current_macro_name = None;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }
    }

    Ok(component_names)
}
