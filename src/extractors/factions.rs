use anyhow::Result;
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;

use crate::parsers::{game_xml, XmlEntry};

/// Extract faction names from game data XML
/// Parses libraries/factions.xml and resolves names via localization
pub fn extract_faction_names(
    xml_entries: &[XmlEntry],
    localization: &HashMap<String, HashMap<String, String>>,
) -> Result<HashMap<String, String>> {
    let mut faction_names = HashMap::new();

    for entry in xml_entries {
        // Look for factions.xml files
        if !entry.name.contains("factions.xml") {
            continue;
        }

        let mut reader = Reader::from_str(&entry.content);
        reader.trim_text(true);
        let mut buf = Vec::new();

        let mut current_faction_id: Option<String> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    match e.name().as_ref() {
                        b"faction" => {
                            // Get faction id attribute
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"id" {
                                    current_faction_id =
                                        Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                            }
                        }
                        b"identification" => {
                            // Get name attribute and resolve via localization
                            if let Some(ref faction_id) = current_faction_id {
                                for attr in e.attributes().flatten() {
                                    if attr.key.as_ref() == b"name" {
                                        let name_ref =
                                            String::from_utf8_lossy(&attr.value).to_string();
                                        let resolved =
                                            game_xml::resolve_name(&name_ref, localization);
                                        let cleaned = game_xml::clean_name(&resolved);
                                        faction_names.insert(faction_id.clone(), cleaned);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"faction" {
                        current_faction_id = None;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }
    }

    Ok(faction_names)
}
