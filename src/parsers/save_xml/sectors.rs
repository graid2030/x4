use anyhow::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

use crate::models::Sector;
use crate::parsers::game_xml::resolve_name;

/// Extract sectors from save file
pub fn extract_sectors(
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
    let mut seen_codes = std::collections::HashSet::new();

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
                    // capture nested identification
                    if e.name().as_ref() == b"identification" {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"name" {
                                    let name_ref = String::from_utf8_lossy(&attr.value).to_string();
                                    pending_ident_name = Some(resolve_name(&name_ref, localization));
                                }
                            }
                        }
                    } else if e.name().as_ref() == b"component" {
                        let mut is_ident = false;
                        let mut name_ref: Option<String> = None;
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"class" && attr.value.as_ref() == b"identification" {
                                    is_ident = true;
                                } else if attr.key.as_ref() == b"name" {
                                    name_ref = Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                            }
                        }
                        if is_ident {
                            if let Some(nr) = name_ref { pending_ident_name = Some(resolve_name(&nr, localization)); }
                        }
                    }
                }

                if e.name().as_ref() == b"component" {
                    let mut is_sector = false;
                    let mut code = None;
                    let mut macro_name = None;
                    let mut own_name_ref: Option<String> = None;
                    let mut owner = None;
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"class" if attr.value.as_ref() == b"sector" => is_sector = true,
                                b"code" => code = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                b"macro" => macro_name = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                b"name" => own_name_ref = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                b"owner" => owner = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                _ => {}
                            }
                        }
                    }
                    if is_sector {
                        if let (Some(c), Some(m)) = (code, macro_name) {
                            in_sector = true;
                            sector_depth = 1;
                            pending_code = Some(c);
                            pending_macro = Some(m);
                            pending_owner = owner;
                            // If sector component itself has a name attribute, resolve it
                            if let Some(nr) = own_name_ref {
                                let resolved = resolve_name(&nr, localization);
                                pending_ident_name = Some(resolved);
                            } else {
                                pending_ident_name = None;
                            }
                        }
                    }
                }
            }
            Ok(Event::Empty(ref e)) => {
                // Treat Empty as Start+End for depth tracking
                if in_sector {
                    // capture identification on self-closing
                    if e.name().as_ref() == b"identification" {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"name" {
                                    let name_ref = String::from_utf8_lossy(&attr.value).to_string();
                                    pending_ident_name = Some(resolve_name(&name_ref, localization));
                                }
                            }
                        }
                    }
                }
                // Also start a sector on Empty component (rare)
                if e.name().as_ref() == b"component" {
                    let mut is_sector = false;
                    let mut code = None;
                    let mut macro_name = None;
                    let mut own_name_ref: Option<String> = None;
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"class" if attr.value.as_ref() == b"sector" => is_sector = true,
                                b"code" => code = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                b"macro" => macro_name = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                b"name" => own_name_ref = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                _ => {}
                            }
                        }
                    }
                    if is_sector {
                        if let (Some(c), Some(m)) = (code, macro_name) {
                            in_sector = true;
                            sector_depth = 1;
                            pending_code = Some(c);
                            pending_macro = Some(m);
                            if let Some(nr) = own_name_ref {
                                let resolved = resolve_name(&nr, localization);
                                pending_ident_name = Some(resolved);
                            }
                        }
                    }
                }
            }
            Ok(Event::End(_)) => {
                if in_sector {
                    if sector_depth > 0 { sector_depth -= 1; }
                    if sector_depth == 0 {
                        if let (Some(c), Some(m)) = (pending_code.take(), pending_macro.take()) {
                            if !seen_codes.contains(&c) {
                                seen_codes.insert(c.clone());
                                let key = m.to_lowercase();
                                let name_code = sector_code_names.get(&c).cloned();
                                let name_ds = sector_names.get(&key).cloned();
                                let name_macro = component_names.get(&key).cloned();
                                let chosen = pending_ident_name
                                    .clone()
                                    .or(name_code.clone())
                                    .or(name_ds.clone())
                                    .or(name_macro.clone())
                                    .unwrap_or_else(|| c.clone());
                                let chosen = crate::parsers::game_xml::clean_name(&chosen);
                                if c == "AAM-257" {
                                    eprintln!(
                                        "DIAG AAM-257: macro={} has_sector_name={} has_component_name={} ident={:?} name_code={:?} name_ds={:?} name_macro={:?} chosen={}",
                                        m,
                                        sector_names.contains_key(&key),
                                        component_names.contains_key(&key),
                                        pending_ident_name,
                                        name_code,
                                        name_ds,
                                        name_macro,
                                        chosen
                                    );
                                }
                                sectors.push(Sector { code: c, macro_name: m, name: chosen, owner: pending_owner.clone() });
                            }
                        }
                        in_sector = false;
                        pending_ident_name = None;
                        pending_owner = None;
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow::anyhow!("XML parse error: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    // Fallback: if somehow nothing was captured, try simple scan (older logic)
    if sectors.is_empty() {
        let mut reader2 = Reader::from_str(xml_content);
        reader2.trim_text(true);
        let mut buf2 = Vec::new();
        let mut seen = std::collections::HashSet::new();
        loop {
            match reader2.read_event_into(&mut buf2) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    if e.name().as_ref() == b"component" {
                        let mut is_sector = false;
                        let mut code = None;
                        let mut macro_name = None;
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                match attr.key.as_ref() {
                                    b"class" if attr.value.as_ref() == b"sector" => is_sector = true,
                                    b"code" => code = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                    b"macro" => macro_name = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                    _ => {}
                                }
                            }
                        }
                        if is_sector {
                            if let (Some(c), Some(m)) = (code, macro_name) {
                                if !seen.contains(&c) {
                                    seen.insert(c.clone());
                                    let key = m.to_lowercase();
                                    let name = sector_code_names
                                        .get(&c)
                                        .cloned()
                                        .or_else(|| sector_names.get(&key).cloned())
                                        .or_else(|| component_names.get(&key).cloned())
                                        .unwrap_or_else(|| c.clone());
                                    sectors.push(Sector { code: c, macro_name: m, name, owner: None });
                                }
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf2.clear();
        }
    }

    sectors.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(sectors)
}
