use anyhow::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use std::collections::HashMap;

/// Resolve localized names like {20104,1001} using the localization map
pub fn resolve_name(identity: &str, localization: &HashMap<String, HashMap<String, String>>) -> String {
    let re = Regex::new(r"\{([^}]+)\}").unwrap();
    let mut result = identity.to_string();

    for cap in re.captures_iter(identity) {
        let segment = &cap[0];
        let inner = &cap[1];

        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 2 {
            let page_id = parts[0].trim();
            let entry_id = parts[1].trim();
            if let Some(page) = localization.get(page_id) {
                if let Some(text) = page.get(entry_id) {
                    let resolved = resolve_name(text, localization);
                    result = result.replace(segment, &resolved);
                    continue;
                }
            }
        }

        result = result.replace(segment, inner.trim());
    }

    result
}

/// Post-process resolved names to remove voice hints and duplicate parentheses
pub fn clean_name(input: &str) -> String {
    // Remove parenthetical hints like (speak as ...), (voice: ...)
    let mut s = input.to_string();
    let hint_re = Regex::new(r"\((?i:speak[^)]*|voice:[^)]*)\)").unwrap();
    s = hint_re.replace_all(&s, "").to_string();

    // Collapse duplicate patterns like "Name(Name)" or "Name (Name)"
    // Loop until no more duplicates are found
    loop {
        let before = s.clone();

        let outer_re = Regex::new(r"^\s*([^()]+)").unwrap();
        if let Some(cap) = outer_re.captures(&s) {
            let base = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            if !base.is_empty() {
                let pattern = format!(r"\(\s*{}\s*\)", regex::escape(base));
                let dup_re = Regex::new(&pattern).unwrap();
                s = dup_re.replace_all(&s, "").to_string();
            }
        }

        // If nothing changed, we're done
        if s == before {
            break;
        }
    }

    // Remove redundant spaces and stray parentheses/commas
    let space_re = Regex::new(r"\s{2,}").unwrap();
    s = s.replace("()", "");
    s = space_re.replace_all(&s, " ").to_string();
    s.trim().to_string()
}

/// Extract localization data from X4 language XML files
pub fn extract_localization(xml_content: &str, lang_id: &str) -> Result<HashMap<String, HashMap<String, String>>> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut localization = HashMap::new();
    let mut buf = Vec::new();

    let mut current_page_id: Option<String> = None;
    let mut file_lang_id: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                match e.name().as_ref() {
                    b"language" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"id" {
                                    file_lang_id = Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                            }
                        }
                    }
                    b"page" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"id" {
                                    current_page_id = Some(String::from_utf8_lossy(&attr.value).to_string());
                                    if let Some(page_id) = &current_page_id {
                                        localization.entry(page_id.clone()).or_insert_with(HashMap::new);
                                    }
                                }
                            }
                        }
                    }
                    b"t" => {
                        // Accept entries for matching language; if file omits id, accept as well
                        if file_lang_id.as_deref() == Some(lang_id) || file_lang_id.is_none() {
                            if let Some(page_id) = &current_page_id {
                                let mut t_id = None;
                                for attr in e.attributes() {
                                    if let Ok(attr) = attr {
                                        if attr.key.as_ref() == b"id" {
                                            t_id = Some(String::from_utf8_lossy(&attr.value).to_string());
                                        }
                                    }
                                }

                                if let Some(tid) = t_id {
                                    if let Ok(Event::Text(text)) = reader.read_event_into(&mut buf) {
                                        let content = text.unescape().unwrap_or_default().to_string();
                                        if let Some(page) = localization.get_mut(page_id) {
                                            page.insert(tid, content);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow::anyhow!("XML parse error: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(localization)
}
