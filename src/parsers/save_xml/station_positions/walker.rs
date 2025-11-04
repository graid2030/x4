use anyhow::{anyhow, Result};
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;

use crate::models::StationInfo;
use crate::parsers::save_xml::helpers::build_factory_name;

pub(super) fn walk_component(
    reader: &mut Reader<&[u8]>,
    buf: &mut Vec<u8>,
    acc: (f64, f64, f64),
    stations: &mut Vec<StationInfo>,
    component_names: &HashMap<String, String>,
    faction_names: &HashMap<String, String>,
    station_code: Option<String>,
    station_macro: Option<String>,
    station_owner: Option<String>,
    station_custom_name: Option<String>,
    is_station: bool,
) -> Result<()> {
    let mut component_depth = 1;
    let mut in_offset = false;
    let mut in_connections = false;
    let mut in_connection = false;

    let mut current_offset = (0.0, 0.0, 0.0);
    let mut source_entry: Option<String> = None;
    let mut source_nameindex: Option<u32> = None;

    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                match e.name().as_ref() {
                    b"component" => {
                        component_depth += 1;

                        // Parse nested component attributes
                        if in_connections && in_connection {
                            // Nested component inside connection - recursively process
                            let mut nested_code = None;
                            let mut nested_class = None;
                            let mut nested_macro = None;
                            let mut nested_owner = None;
                            let mut nested_name = None;

                            for attr in e.attributes().flatten() {
                                match attr.key.as_ref() {
                                    b"class" => {
                                        nested_class =
                                            Some(String::from_utf8_lossy(&attr.value).to_string())
                                    }
                                    b"code" => {
                                        nested_code =
                                            Some(String::from_utf8_lossy(&attr.value).to_string())
                                    }
                                    b"macro" => {
                                        nested_macro =
                                            Some(String::from_utf8_lossy(&attr.value).to_string())
                                    }
                                    b"owner" => {
                                        nested_owner =
                                            Some(String::from_utf8_lossy(&attr.value).to_string())
                                    }
                                    b"name" => {
                                        nested_name =
                                            Some(String::from_utf8_lossy(&attr.value).to_string())
                                    }
                                    _ => {}
                                }
                            }

                            // If it's a station, recursively walk it with accumulated position
                            if nested_class.as_deref() == Some("station") {
                                let acc2 = (
                                    acc.0 + current_offset.0,
                                    acc.1 + current_offset.1,
                                    acc.2 + current_offset.2,
                                );
                                walk_component(
                                    reader,
                                    buf,
                                    acc2,
                                    stations,
                                    component_names,
                                    faction_names,
                                    nested_code,
                                    nested_macro,
                                    nested_owner,
                                    nested_name,
                                    true,
                                )?;
                                component_depth -= 1;
                                continue;
                            }
                        }
                    }
                    b"offset" => in_offset = true,
                    b"position" if in_offset => {
                        // Read position inside offset
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"x" => {
                                    current_offset.0 =
                                        String::from_utf8_lossy(&attr.value).parse().unwrap_or(0.0)
                                }
                                b"y" => {
                                    current_offset.1 =
                                        String::from_utf8_lossy(&attr.value).parse().unwrap_or(0.0)
                                }
                                b"z" => {
                                    current_offset.2 =
                                        String::from_utf8_lossy(&attr.value).parse().unwrap_or(0.0)
                                }
                                _ => {}
                            }
                        }
                    }
                    b"source" => {
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"entry" => {
                                    source_entry =
                                        Some(String::from_utf8_lossy(&attr.value).to_string())
                                }
                                b"nameindex" => {
                                    source_nameindex =
                                        String::from_utf8_lossy(&attr.value).parse().ok()
                                }
                                _ => {}
                            }
                        }
                    }
                    b"connections" => in_connections = true,
                    b"connection" if in_connections => in_connection = true,
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => match e.name().as_ref() {
                b"component" => {
                    component_depth -= 1;
                    if component_depth == 0 {
                        // Exiting the component we're walking
                        if is_station {
                            // Build and save station with accumulated position
                            if let (Some(code), Some(macro_name)) =
                                (station_code.clone(), station_macro.clone())
                            {
                                let acc2 = (
                                    acc.0 + current_offset.0,
                                    acc.1 + current_offset.1,
                                    acc.2 + current_offset.2,
                                );

                                let name = if let Some(custom) = station_custom_name.clone() {
                                    custom
                                } else if let Some(entry) = source_entry.clone() {
                                    build_factory_name(&entry, source_nameindex.unwrap_or(1))
                                } else {
                                    component_names
                                        .get(&macro_name.to_lowercase())
                                        .cloned()
                                        .unwrap_or_else(|| macro_name.clone())
                                };

                                let owner_name = station_owner.as_ref().map(|o| {
                                    if let Some(name) = faction_names.get(o.as_str()) {
                                        name.clone()
                                    } else {
                                        let mut chars = o.chars();
                                        match chars.next() {
                                            Some(first) => {
                                                first.to_uppercase().chain(chars).collect()
                                            }
                                            None => o.clone(),
                                        }
                                    }
                                });

                                stations.push(StationInfo {
                                    code,
                                    name,
                                    owner: station_owner.clone(),
                                    owner_name,
                                    macro_name,
                                    position: acc2,
                                });
                            }
                        }
                        return Ok(());
                    }
                }
                b"offset" => in_offset = false,
                b"connections" => in_connections = false,
                b"connection" => in_connection = false,
                _ => {}
            },
            Ok(Event::Eof) => return Ok(()),
            Err(e) => return Err(anyhow!("XML parse error: {e}")),
            _ => {}
        }
        buf.clear();
    }
}
