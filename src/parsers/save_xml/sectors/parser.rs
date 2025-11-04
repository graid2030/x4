use anyhow::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::{HashMap, HashSet};

use crate::models::Sector;
use crate::parsers::game_xml::resolve_name;

pub(super) fn parse_sectors(
    xml_content: &str,
    sector_names: &HashMap<String, String>,
    component_names: &HashMap<String, String>,
    sector_code_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
) -> Result<Vec<Sector>> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut sectors = Vec::new();
    let mut buf = Vec::new();
    let mut seen_codes = HashSet::new();

    // Track when we are inside a sector component to catch nested identification name
    let mut in_sector = false;
    let mut sector_depth: usize = 0;
    let mut pending_code: Option<String> = None;
    let mut pending_macro: Option<String> = None;
    let mut pending_ident_name: Option<String> = None;
    let mut pending_owner: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if in_sector {
                    sector_depth += 1;
                    if e.name().as_ref() == b"identification" {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                let name_ref = String::from_utf8_lossy(&attr.value).to_string();
                                pending_ident_name = Some(resolve_name(&name_ref, localization));
                            }
                        }
                    } else if e.name().as_ref() == b"component" {
                        let mut is_ident = false;
                        let mut name_ref: Option<String> = None;
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"class"
                                && attr.value.as_ref() == b"identification"
                            {
                                is_ident = true;
                            } else if attr.key.as_ref() == b"name" {
                                name_ref = Some(String::from_utf8_lossy(&attr.value).to_string());
                            }
                        }
                        if is_ident {
                            if let Some(nr) = name_ref {
                                pending_ident_name = Some(resolve_name(&nr, localization));
                            }
                        }
                    }
                }

                if e.name().as_ref() == b"component" {
                    let mut is_sector = false;
                    let mut code = None;
                    let mut macro_name = None;
                    let mut owner = None;

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"class" if attr.value.as_ref() == b"sector" => is_sector = true,
                            b"code" => {
                                code = Some(String::from_utf8_lossy(&attr.value).to_string())
                            }
                            b"macro" => {
                                macro_name = Some(String::from_utf8_lossy(&attr.value).to_string())
                            }
                            b"owner" => {
                                owner = Some(String::from_utf8_lossy(&attr.value).to_string())
                            }
                            _ => {}
                        }
                    }

                    if is_sector {
                        in_sector = true;
                        sector_depth = 0;
                        pending_code = code.clone();
                        pending_macro = macro_name.clone();
                        pending_owner = owner.clone();

                        if let (Some(c), Some(m)) = (code, macro_name) {
                            if seen_codes.insert(c.clone()) {
                                let key = m.to_lowercase();
                                let name_macro = sector_names
                                    .get(&key)
                                    .cloned()
                                    .or_else(|| component_names.get(&key).cloned());
                                let name_ds = sector_code_names.get(&c).cloned();
                                let chosen = pending_ident_name
                                    .clone()
                                    .or(name_ds.clone())
                                    .or(name_macro.clone())
                                    .unwrap_or_else(|| c.clone());

                                sectors.push(Sector {
                                    code: c.clone(),
                                    macro_name: m.clone(),
                                    name: chosen,
                                    owner,
                                });
                            }
                        }
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                if in_sector {
                    if sector_depth == 0 && e.name().as_ref() == b"component" {
                        if let (Some(c), Some(m)) = (pending_code.clone(), pending_macro.clone()) {
                            if !sectors.iter().any(|s| s.code == c) {
                                let key = m.to_lowercase();
                                let name_macro = sector_names
                                    .get(&key)
                                    .cloned()
                                    .or_else(|| component_names.get(&key).cloned());
                                let name_ds = sector_code_names.get(&c).cloned();
                                let chosen = pending_ident_name
                                    .clone()
                                    .or(name_ds.clone())
                                    .or(name_macro.clone())
                                    .unwrap_or_else(|| c.clone());

                                sectors.push(Sector {
                                    code: c,
                                    macro_name: m,
                                    name: chosen,
                                    owner: pending_owner.clone(),
                                });
                            }
                        }
                        in_sector = false;
                        pending_ident_name = None;
                        pending_owner = None;
                    } else if sector_depth > 0 {
                        sector_depth -= 1;
                    }
                }
            }
            Ok(Event::Empty(ref e)) => {
                if e.name().as_ref() == b"component" {
                    let mut is_sector = false;
                    let mut code = None;
                    let mut macro_name = None;
                    let mut owner = None;

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"class" if attr.value.as_ref() == b"sector" => is_sector = true,
                            b"code" => {
                                code = Some(String::from_utf8_lossy(&attr.value).to_string())
                            }
                            b"macro" => {
                                macro_name = Some(String::from_utf8_lossy(&attr.value).to_string())
                            }
                            b"owner" => {
                                owner = Some(String::from_utf8_lossy(&attr.value).to_string())
                            }
                            _ => {}
                        }
                    }

                    if is_sector {
                        if let (Some(c), Some(m)) = (code, macro_name) {
                            if seen_codes.insert(c.clone()) {
                                let key = m.to_lowercase();
                                let name_macro = sector_names
                                    .get(&key)
                                    .cloned()
                                    .or_else(|| component_names.get(&key).cloned());
                                let name_ds = sector_code_names.get(&c).cloned();
                                let chosen = pending_ident_name
                                    .clone()
                                    .or(name_ds.clone())
                                    .or(name_macro.clone())
                                    .unwrap_or_else(|| c.clone());

                                sectors.push(Sector {
                                    code: c,
                                    macro_name: m,
                                    name: chosen,
                                    owner,
                                });
                            }
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

    Ok(sectors)
}
