use anyhow::Result;
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;

use crate::models::PlayerNpc;
use crate::parsers::game_xml::{clean_name, resolve_name};
use super::helpers::has_owner_player;
use super::types::Ctx;

pub fn extract_player_npcs(
    xml: &str,
    localization: &HashMap<String, HashMap<String, String>>,
) -> Result<Vec<PlayerNpc>> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut stack: Vec<Ctx> = Vec::new();

    let mut npcs: Vec<PlayerNpc> = Vec::new();
    let mut current_id: Option<String> = None;
    let mut current_name: Option<String> = None;
    let mut current_code: Option<String> = None;
    let mut skills: HashMap<String, i32> = HashMap::new();

    let mut in_traits = false;

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
                    if attrs.get("class").map(|s| s.as_str()) == Some("npc")
                        && has_owner_player(&attrs)
                    {
                        current_id = attrs.get("id").cloned();
                        current_code = attrs.get("code").cloned();
                        if let Some(nr) = attrs.get("name") {
                            current_name = Some(clean_name(&resolve_name(nr, localization)));
                        }
                        skills.clear();
                    }
                } else if tag == "traits" {
                    in_traits = current_id.is_some();
                } else if tag == "skill" && in_traits && current_id.is_some() {
                    if let (Some(t), Some(v)) = (attrs.get("type"), attrs.get("value")) {
                        if let Ok(n) = v.parse::<i32>() {
                            skills.insert(t.clone(), n);
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
                    if attrs.get("class").map(|s| s.as_str()) == Some("npc")
                        && has_owner_player(&attrs)
                    {
                        let id = attrs.get("id").cloned();
                        let code = attrs.get("code").cloned();
                        let name = attrs
                            .get("name")
                            .map(|nr| clean_name(&resolve_name(nr, localization)));
                        if let (Some(id), Some(name)) = (id, name) {
                            npcs.push(PlayerNpc {
                                id,
                                name,
                                code,
                                piloting: 0,
                                engineering: 0,
                                boarding: 0,
                                management: 0,
                                morale: 0,
                            });
                        }
                    }
                } else if tag == "skill" && in_traits && current_id.is_some() {
                    if let (Some(t), Some(v)) = (attrs.get("type"), attrs.get("value")) {
                        if let Ok(n) = v.parse::<i32>() {
                            skills.insert(t.clone(), n);
                        }
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if tag == "traits" {
                    in_traits = false;
                }
                if tag == "component" {
                    if let (Some(id), Some(name)) = (current_id.take(), current_name.take()) {
                        let code = current_code.take();
                        let piloting = *skills.get("piloting").unwrap_or(&0);
                        let engineering = *skills.get("engineering").unwrap_or(&0);
                        let boarding = *skills.get("boarding").unwrap_or(&0);
                        let management = *skills.get("management").unwrap_or(&0);
                        let morale = *skills.get("morale").unwrap_or(&0);
                        npcs.push(PlayerNpc {
                            id,
                            name,
                            code,
                            piloting,
                            engineering,
                            boarding,
                            management,
                            morale,
                        });
                    }
                    skills.clear();
                }
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

    Ok(npcs)
}
