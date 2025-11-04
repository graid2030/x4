use anyhow::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

use crate::models::{TransportType, WareMeta, WareMetaMap};
use crate::parsers::XmlEntry;

/// Extract ware metadata (transport type, volume) from XML files
pub fn extract_wares(xml_entries: &[XmlEntry]) -> Result<WareMetaMap> {
    let mut wares = HashMap::new();

    for entry in xml_entries {
        if let Ok(extracted) = extract_wares_from_xml(&entry.content) {
            for (id, meta) in extracted {
                wares.entry(id).or_insert(meta);
            }
        }
    }

    Ok(wares)
}

fn extract_wares_from_xml(xml_content: &str) -> Result<WareMetaMap> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut wares = HashMap::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                if e.name().as_ref() == b"ware" {
                    let mut ware_id = None;
                    let mut transport = None;
                    let mut volume = None;
                    let mut name_ref: Option<String> = None;

                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"id" => {
                                    ware_id = Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                                b"name" => {
                                    name_ref = Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                                b"transport" => {
                                    let t = String::from_utf8_lossy(&attr.value).to_lowercase();
                                    transport = match t.as_str() {
                                        "container" => Some(TransportType::Container),
                                        "solid" => Some(TransportType::Solid),
                                        "liquid" => Some(TransportType::Liquid),
                                        "energy" => Some(TransportType::Energy),
                                        _ => None,
                                    };
                                }
                                b"volume" => {
                                    let v = String::from_utf8_lossy(&attr.value);
                                    volume = v.parse::<f64>().ok();
                                }
                                _ => {}
                            }
                        }
                    }

                    if let (Some(id), Some(t)) = (ware_id, transport) {
                        wares.insert(
                            id.clone(),
                            WareMeta {
                                id,
                                transport: t,
                                volume,
                                name_ref,
                            },
                        );
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => {
                // Skip malformed XML
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(wares)
}
