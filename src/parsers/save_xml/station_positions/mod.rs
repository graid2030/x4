mod walker;

use anyhow::{anyhow, Result};
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;

use crate::models::StationInfo;

use walker::walk_component;

/// Extract stations for a specific sector with their positions.
/// Uses recursive position accumulation as per SaveStructure.md
pub fn extract_stations_for_sector(
    xml_content: &str,
    sector_code: &str,
    component_names: &HashMap<String, String>,
    faction_names: &HashMap<String, String>,
) -> Result<Vec<StationInfo>> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut stations = Vec::new();
    let mut buf = Vec::new();

    // Find the target sector
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"component" => {
                let mut is_sector = false;
                let mut code = None;

                for attr in e.attributes().flatten() {
                    match attr.key.as_ref() {
                        b"class" if attr.value.as_ref() == b"sector" => is_sector = true,
                        b"code" => code = Some(String::from_utf8_lossy(&attr.value).to_string()),
                        _ => {}
                    }
                }

                if is_sector && code.as_ref() == Some(&sector_code.to_string()) {
                    // Found target sector - start recursive walk
                    walk_component(
                        &mut reader,
                        &mut buf,
                        (0.0, 0.0, 0.0),
                        &mut stations,
                        component_names,
                        faction_names,
                        None,
                        None,
                        None,
                        None,
                        false,
                    )?;
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow!("XML parse error: {e}")),
            _ => {}
        }
        buf.clear();
    }

    Ok(stations)
}
