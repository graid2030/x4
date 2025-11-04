use crate::models::{Position, SectorMapData};
use anyhow::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

use super::component_processor::process_component_end;
use super::types::StackItem;

/// Extract sector map data from save XML
pub fn extract_sector_map(
    xml_content: &str,
    sector_code: &str,
    sector_names: &HashMap<String, String>,
    component_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
    sector_owner: Option<String>,
) -> Result<SectorMapData> {
    eprintln!(
        "[SECTOR_MAP] Starting extraction for sector_code: '{}'",
        sector_code
    );
    eprintln!(
        "[SECTOR_MAP] XML content length: {} bytes",
        xml_content.len()
    );

    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut stations = Vec::new();
    let mut ships = Vec::new();
    let mut gates = Vec::new();
    let mut asteroids = Vec::new();
    let mut resources = Vec::new();

    let mut in_target_sector = false;
    let mut stack: Vec<StackItem> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = HashMap::new();

                for attr in e.attributes() {
                    if let Ok(attr) = attr {
                        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                        let value = String::from_utf8_lossy(&attr.value).to_string();
                        attrs.insert(key, value);
                    }
                }

                // Check if entering target sector
                if !in_target_sector
                    && tag == "component"
                    && attrs.get("class") == Some(&"sector".to_string())
                    && attrs.get("code") == Some(&sector_code.to_string())
                {
                    in_target_sector = true;
                    eprintln!("[SECTOR_MAP] Found target sector: {}", sector_code);
                }

                if in_target_sector {
                    stack.push(StackItem {
                        tag,
                        attrs,
                        position: None,
                    });
                }
            }
            Ok(Event::Empty(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();

                if in_target_sector && tag == "position" && !stack.is_empty() {
                    let mut x = 0.0;
                    let mut y = 0.0;
                    let mut z = 0.0;

                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"x" => {
                                    x = String::from_utf8_lossy(&attr.value)
                                        .parse()
                                        .unwrap_or(0.0)
                                }
                                b"y" => {
                                    y = String::from_utf8_lossy(&attr.value)
                                        .parse()
                                        .unwrap_or(0.0)
                                }
                                b"z" => {
                                    z = String::from_utf8_lossy(&attr.value)
                                        .parse()
                                        .unwrap_or(0.0)
                                }
                                _ => {}
                            }
                        }
                    }

                    // Find the last component in stack and set its position
                    for item in stack.iter_mut().rev() {
                        if item.tag == "component" {
                            item.position = Some(Position { x, y, z });
                            break;
                        }
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();

                if in_target_sector && tag == "component" && !stack.is_empty() {
                    process_component_end(
                        &mut stack,
                        sector_code,
                        component_names,
                        localization,
                        &mut stations,
                        &mut ships,
                        &mut gates,
                        &mut asteroids,
                        &mut resources,
                        &mut in_target_sector,
                    );
                } else if in_target_sector && !stack.is_empty() {
                    // Remove last matching tag from stack
                    for i in (0..stack.len()).rev() {
                        if stack[i].tag == tag {
                            stack.remove(i);
                            break;
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow::anyhow!("XML parse error: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    let sector_name = sector_names
        .get(sector_code)
        .cloned()
        .unwrap_or_else(|| sector_code.to_string());

    Ok(SectorMapData {
        sector_code: sector_code.to_string(),
        sector_name,
        owner: sector_owner,
        stations,
        ships,
        gates,
        asteroids,
        resources,
    })
}
