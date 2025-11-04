use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::{HashMap, HashSet};

use crate::models::Sector;

pub(super) fn fallback_scan(
    xml_content: &str,
    sector_names: &HashMap<String, String>,
    component_names: &HashMap<String, String>,
    sector_code_names: &HashMap<String, String>,
) -> Vec<Sector> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut sectors = Vec::new();
    let mut seen = HashSet::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                if e.name().as_ref() == b"component" {
                    let mut is_sector = false;
                    let mut code = None;
                    let mut macro_name = None;

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"class" if attr.value.as_ref() == b"sector" => is_sector = true,
                            b"code" => {
                                code = Some(String::from_utf8_lossy(&attr.value).to_string())
                            }
                            b"macro" => {
                                macro_name = Some(String::from_utf8_lossy(&attr.value).to_string())
                            }
                            _ => {}
                        }
                    }

                    if is_sector {
                        if let (Some(c), Some(m)) = (code, macro_name) {
                            if seen.insert(c.clone()) {
                                let key = m.to_lowercase();
                                let name = sector_code_names
                                    .get(&c)
                                    .cloned()
                                    .or_else(|| sector_names.get(&key).cloned())
                                    .or_else(|| component_names.get(&key).cloned())
                                    .unwrap_or_else(|| c.clone());

                                sectors.push(Sector {
                                    code: c,
                                    macro_name: m,
                                    name,
                                    owner: None,
                                });
                            }
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    sectors
}
