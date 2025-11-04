use quick_xml::events::{attributes::Attributes, BytesStart};
use std::collections::HashMap;

use crate::models::PilotInfo;
use crate::parsers::game_xml::{clean_name, resolve_name};

pub struct PilotState<'a> {
    localization: &'a HashMap<String, HashMap<String, String>>,
    pub name: Option<String>,
    pub credits: Option<i64>,
    pub location: Option<String>,
    in_player_faction: bool,
    depth: usize,
}

impl<'a> PilotState<'a> {
    pub fn new(localization: &'a HashMap<String, HashMap<String, String>>) -> Self {
        Self {
            localization,
            name: None,
            credits: None,
            location: None,
            in_player_faction: false,
            depth: 0,
        }
    }

    pub fn handle_start(&mut self, e: &BytesStart<'_>) {
        let tag = e.name().as_ref().to_vec();
        self.depth += 1;

        if tag.as_slice() == b"player" {
            self.capture_player_attrs(e.attributes());
        }

        if tag.as_slice() == b"faction" {
            for attr in e.attributes().flatten() {
                if attr.key.as_ref() == b"id" && attr.value.as_ref() == b"player" {
                    self.in_player_faction = true;
                }
            }
        }

        if self.in_player_faction
            && (tag.as_slice() == b"identification" || tag.as_slice() == b"component")
        {
            let mut is_ident = tag.as_slice() == b"identification";
            let mut name_ref: Option<String> = None;
            for attr in e.attributes().flatten() {
                if attr.key.as_ref() == b"name" {
                    name_ref = Some(String::from_utf8_lossy(&attr.value).to_string());
                }
                if attr.key.as_ref() == b"class" && attr.value.as_ref() == b"identification" {
                    is_ident = true;
                }
            }
            if is_ident {
                if let Some(nr) = name_ref {
                    let resolved = resolve_name(&nr, self.localization);
                    if !resolved.is_empty() {
                        self.name = Some(resolved);
                    }
                }
            }
        }

        self.collect_credits(tag.as_slice(), e.attributes());
    }

    pub fn handle_empty(&mut self, e: &BytesStart<'_>) {
        let tag = e.name().as_ref().to_vec();

        if tag.as_slice() == b"player" {
            self.capture_player_attrs(e.attributes());
        }

        if tag.as_slice() == b"faction" {
            for attr in e.attributes().flatten() {
                if attr.key.as_ref() == b"id" && attr.value.as_ref() == b"player" {
                    self.in_player_faction = true;
                }
            }
        }

        if self.in_player_faction
            && (tag.as_slice() == b"identification" || tag.as_slice() == b"component")
        {
            let mut is_ident = tag.as_slice() == b"identification";
            let mut name_ref: Option<String> = None;
            for attr in e.attributes().flatten() {
                if attr.key.as_ref() == b"name" {
                    name_ref = Some(String::from_utf8_lossy(&attr.value).to_string());
                }
                if attr.key.as_ref() == b"class" && attr.value.as_ref() == b"identification" {
                    is_ident = true;
                }
            }
            if is_ident {
                if let Some(nr) = name_ref {
                    let resolved = resolve_name(&nr, self.localization);
                    if !resolved.is_empty() {
                        self.name = Some(resolved);
                    }
                }
            }
        }

        self.collect_credits(tag.as_slice(), e.attributes());
    }

    pub fn handle_end(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
        if self.in_player_faction && self.depth == 0 {
            self.in_player_faction = false;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.name.is_some() && self.credits.is_some()
    }

    pub fn into_info(self) -> PilotInfo {
        PilotInfo {
            name: self.name.unwrap_or_else(|| "Player".to_string()),
            credits: self.credits.unwrap_or(0),
            location: self.location,
        }
    }

    fn capture_player_attrs(&mut self, attributes: Attributes<'_>) {
        for attr in attributes.flatten() {
            match attr.key.as_ref() {
                b"name" => {
                    let v = String::from_utf8_lossy(&attr.value).to_string();
                    if !v.is_empty() {
                        self.name = Some(v);
                    }
                }
                b"location" => {
                    let v = String::from_utf8_lossy(&attr.value).to_string();
                    let resolved = resolve_name(&v, self.localization);
                    let cleaned = clean_name(&resolved);
                    if !cleaned.is_empty() {
                        self.location = Some(cleaned);
                    }
                }
                b"money" | b"funds" | b"value" | b"amount" | b"credits" | b"balance"
                | b"budget" => {
                    if let Ok(s) = std::str::from_utf8(&attr.value) {
                        if let Ok(n) = s.parse::<i64>() {
                            self.credits = Some(n);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_credits(&mut self, tag: &[u8], attributes: Attributes<'_>) {
        let mut owner_is_player = false;
        let mut val: Option<i64> = None;
        for attr in attributes.flatten() {
            let key = attr.key.as_ref();
            if key == b"owner" || key == b"id" || key == b"faction" {
                if attr.value.as_ref() == b"player" {
                    owner_is_player = true;
                }
            }
            if matches!(
                key,
                b"amount"
                    | b"money"
                    | b"funds"
                    | b"value"
                    | b"credits"
                    | b"credit"
                    | b"balance"
                    | b"budget"
            ) {
                if let Ok(s) = std::str::from_utf8(&attr.value) {
                    if let Ok(n) = s.parse::<i64>() {
                        val = Some(n);
                    }
                }
            }
        }
        if owner_is_player || self.in_player_faction || tag == b"player" {
            if let Some(v) = val {
                self.credits = Some(v);
            }
        }
    }
}
