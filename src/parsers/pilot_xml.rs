use anyhow::Result;
use quick_xml::{events::Event, Reader};

use crate::models::PilotInfo;
use crate::parsers::game_xml::{resolve_name, clean_name};

/// Extract basic pilot info (player name and credits) from save
pub fn extract_pilot_info(
    xml_content: &str,
    localization: &std::collections::HashMap<String, std::collections::HashMap<String, String>>,
) -> Result<PilotInfo> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut name: Option<String> = None;
    let mut credits: Option<i64> = None;
    let mut location: Option<String> = None;

    let mut in_player_faction = false;
    let mut depth: usize = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = e.name().as_ref().to_vec();
                depth += 1;

                // Direct player node under <info>
                if tag.as_slice() == b"player" {
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"name" => {
                                    let v = String::from_utf8_lossy(&attr.value).to_string();
                                    if !v.is_empty() { name = Some(v); }
                                }
                                b"location" => {
                                    let v = String::from_utf8_lossy(&attr.value).to_string();
                                    let resolved = resolve_name(&v, localization);
                                    let cleaned = clean_name(&resolved);
                                    if !cleaned.is_empty() { location = Some(cleaned); }
                                }
                                b"money" | b"funds" | b"value" | b"amount" | b"credits" | b"balance" | b"budget" => {
                                    if let Ok(s) = std::str::from_utf8(&attr.value) {
                                        if let Ok(n) = s.parse::<i64>() { credits = Some(n); }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                if tag.as_slice() == b"faction" {
                    let mut is_player = false;
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            if attr.key.as_ref() == b"id" && attr.value.as_ref() == b"player" { is_player = true; }
                        }
                    }
                    if is_player { in_player_faction = true; }
                }

                if in_player_faction && (tag.as_slice() == b"identification" || tag.as_slice() == b"component") {
                    let mut is_ident = tag == b"identification";
                    let mut name_ref: Option<String> = None;
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            if attr.key.as_ref() == b"name" { name_ref = Some(String::from_utf8_lossy(&attr.value).to_string()); }
                            if attr.key.as_ref() == b"class" && attr.value.as_ref() == b"identification" { is_ident = true; }
                        }
                    }
                    if is_ident {
                        if let Some(nr) = name_ref {
                            let resolved = resolve_name(&nr, localization);
                            if !resolved.is_empty() { name = Some(resolved); }
                        }
                    }
                }

                // Credits detection within player faction
                {
                    let mut owner_is_player = false;
                    let mut val: Option<i64> = None;
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            let key = attr.key.as_ref();
                            if key == b"owner" || key == b"id" || key == b"faction" {
                                if attr.value.as_ref() == b"player" { owner_is_player = true; }
                            }
                            if matches!(key, b"amount" | b"money" | b"funds" | b"value" | b"credits" | b"credit" | b"balance" | b"budget") {
                                if let Ok(s) = std::str::from_utf8(&attr.value) { if let Ok(n) = s.parse::<i64>() { val = Some(n); } }
                            }
                        }
                    }
                    if owner_is_player || in_player_faction || tag.as_slice() == b"player" {
                        if let Some(v) = val { credits = Some(v); }
                    }
                }
            }
            Ok(Event::Empty(ref e)) => {
                let tag = e.name().as_ref().to_vec();

                // Direct player node under <info> (empty)
                if tag.as_slice() == b"player" {
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"name" => {
                                    let v = String::from_utf8_lossy(&attr.value).to_string();
                                    if !v.is_empty() { name = Some(v); }
                                }
                                b"location" => {
                                    let v = String::from_utf8_lossy(&attr.value).to_string();
                                    let resolved = resolve_name(&v, localization);
                                    let cleaned = clean_name(&resolved);
                                    if !cleaned.is_empty() { location = Some(cleaned); }
                                }
                                b"money" | b"funds" | b"value" | b"amount" | b"credits" | b"balance" | b"budget" => {
                                    if let Ok(s) = std::str::from_utf8(&attr.value) {
                                        if let Ok(n) = s.parse::<i64>() { credits = Some(n); }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                if tag.as_slice() == b"faction" {
                    let mut is_player = false;
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            if attr.key.as_ref() == b"id" && attr.value.as_ref() == b"player" { is_player = true; }
                        }
                    }
                    if is_player { in_player_faction = true; }
                }

                if in_player_faction && (tag.as_slice() == b"identification" || tag.as_slice() == b"component") {
                    let mut is_ident = tag == b"identification";
                    let mut name_ref: Option<String> = None;
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            if attr.key.as_ref() == b"name" { name_ref = Some(String::from_utf8_lossy(&attr.value).to_string()); }
                            if attr.key.as_ref() == b"class" && attr.value.as_ref() == b"identification" { is_ident = true; }
                        }
                    }
                    if is_ident {
                        if let Some(nr) = name_ref {
                            let resolved = resolve_name(&nr, localization);
                            if !resolved.is_empty() { name = Some(resolved); }
                        }
                    }
                }

                // Credits detection for empty elements as well
                {
                    let mut owner_is_player = false;
                    let mut val: Option<i64> = None;
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            let key = attr.key.as_ref();
                            if key == b"owner" || key == b"id" || key == b"faction" {
                                if attr.value.as_ref() == b"player" { owner_is_player = true; }
                            }
                            if matches!(key, b"amount" | b"money" | b"funds" | b"value" | b"credits" | b"credit" | b"balance" | b"budget") {
                                if let Ok(s) = std::str::from_utf8(&attr.value) { if let Ok(n) = s.parse::<i64>() { val = Some(n); } }
                            }
                        }
                    }
                    if owner_is_player || in_player_faction || tag.as_slice() == b"player" {
                        if let Some(v) = val { credits = Some(v); }
                    }
                }
            }
            Ok(Event::End(_)) => {
                if depth > 0 { depth -= 1; }
                if in_player_faction && depth == 0 { in_player_faction = false; }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        if name.is_some() && credits.is_some() { break; }
        buf.clear();
    }

    Ok(PilotInfo { name: name.unwrap_or_else(|| "Player".to_string()), credits: credits.unwrap_or(0), location })
}
