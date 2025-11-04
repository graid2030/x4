use anyhow::Result;
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;

use crate::models::PlayerAsset;
use super::helpers::{
    text_attr, has_owner_player, is_station_or_ship,
    resolve_component_name, get_sector_from_stack, derive_ship_meta
};
use super::types::Ctx;

pub fn extract_player_assets(
    xml: &str,
    sector_names: &HashMap<String, String>,
    component_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
) -> Result<Vec<PlayerAsset>> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut stack: Vec<Ctx> = Vec::new();
    let mut assets = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = HashMap::new();
                for a in e.attributes() {
                    if let Ok(a) = a {
                        let k = String::from_utf8_lossy(a.key.as_ref()).to_string();
                        let v = String::from_utf8_lossy(&a.value).to_string();
                        attrs.insert(k, v);
                    }
                }
                stack.push(Ctx {
                    tag: tag.clone(),
                    attrs: attrs.clone(),
                });

                if tag == "component" {
                    if let (Some(class), true) = (attrs.get("class"), has_owner_player(&attrs)) {
                        if is_station_or_ship(class) {
                            let id = text_attr(&attrs, "id").unwrap_or_else(|| "".into());
                            let name = resolve_component_name(&attrs, component_names, localization);
                            let code = text_attr(&attrs, "code");
                            let macro_name = text_attr(&attrs, "macro");
                            let (sector_code, sector_name) =
                                get_sector_from_stack(&stack, sector_names);
                            let (cargo_capacity, cargo_type, ship_type, speed) = derive_ship_meta(
                                macro_name.as_deref(),
                                attrs.get("thruster").map(|s| s.as_str()),
                            );
                            assets.push(PlayerAsset {
                                id,
                                class: class.clone(),
                                name,
                                code,
                                macro_name,
                                cargo_capacity,
                                cargo_type,
                                ship_type,
                                speed,
                                sector_code,
                                sector_name,
                            });
                        }
                    }
                }
            }
            Ok(Event::Empty(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = HashMap::new();
                for a in e.attributes() {
                    if let Ok(a) = a {
                        let k = String::from_utf8_lossy(a.key.as_ref()).to_string();
                        let v = String::from_utf8_lossy(&a.value).to_string();
                        attrs.insert(k, v);
                    }
                }
                if tag == "component" {
                    if let (Some(class), true) = (attrs.get("class"), has_owner_player(&attrs)) {
                        if is_station_or_ship(class) {
                            let id = text_attr(&attrs, "id").unwrap_or_else(|| "".into());
                            let name = resolve_component_name(&attrs, component_names, localization);
                            let code = text_attr(&attrs, "code");
                            let macro_name = text_attr(&attrs, "macro");
                            let (sector_code, sector_name) =
                                get_sector_from_stack(&stack, sector_names);
                            let (cargo_capacity, cargo_type, ship_type, speed) = derive_ship_meta(
                                macro_name.as_deref(),
                                attrs.get("thruster").map(|s| s.as_str()),
                            );
                            assets.push(PlayerAsset {
                                id,
                                class: class.clone(),
                                name,
                                code,
                                macro_name,
                                cargo_capacity,
                                cargo_type,
                                ship_type,
                                speed,
                                sector_code,
                                sector_name,
                            });
                        }
                    }
                }
            }
            Ok(Event::End(_)) => {
                if !stack.is_empty() {
                    stack.pop();
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow::anyhow!("XML parse error: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(assets)
}
