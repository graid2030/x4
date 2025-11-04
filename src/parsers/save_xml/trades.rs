use anyhow::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

use crate::models::{StationWare, TradeType};
use super::helpers::{Context, find_parent_sector, find_parent_station, get_station_name, get_sector_name};

/// Extract all trade offers - completely rewritten to match Python logic
pub fn extract_all_trades(
    xml_content: &str,
    sector_names: &HashMap<String, String>,
    component_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
    sectors: &[crate::models::Sector],
) -> Result<HashMap<String, Vec<StationWare>>> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut trades_by_sector: HashMap<String, Vec<StationWare>> = HashMap::new();
    let mut buf = Vec::new();

    // Create sector code to owner lookup map
    let sector_owners: HashMap<String, Option<String>> = sectors
        .iter()
        .map(|s| (s.code.clone(), s.owner.clone()))
        .collect();

    let mut stack: Vec<Context> = Vec::new();
    let mut depth = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = HashMap::new();

                for attr in e.attributes() {
                    if let Ok(attr) = attr {
                        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                        let value = String::from_utf8_lossy(&attr.value).to_string();
                        attrs.insert(key, value);
                    }
                }

                stack.push(Context {
                    tag: tag.clone(),
                    attrs: attrs.clone(),
                    depth,
                });

                // Process trade elements - ONLY if they're inside a station
                if tag == "trade" {
                    process_trade_element(&attrs, &stack, &sector_names, &component_names, &localization, &sector_owners, &mut trades_by_sector);
                }

                depth += 1;
            }
            Ok(Event::Empty(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = HashMap::new();

                for attr in e.attributes() {
                    if let Ok(attr) = attr {
                        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                        let value = String::from_utf8_lossy(&attr.value).to_string();
                        attrs.insert(key, value);
                    }
                }

                // Also handle self-closing trade elements - ONLY if they're inside a station
                if tag == "trade" {
                    process_trade_element(&attrs, &stack, &sector_names, &component_names, &localization, &sector_owners, &mut trades_by_sector);
                }
            }
            Ok(Event::End(_)) => {
                depth -= 1;
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

    Ok(trades_by_sector)
}

fn process_trade_element(
    attrs: &HashMap<String, String>,
    stack: &[Context],
    sector_names: &HashMap<String, String>,
    component_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
    sector_owners: &HashMap<String, Option<String>>,
    trades_by_sector: &mut HashMap<String, Vec<StationWare>>,
) {
    // First check if we're inside a station
    if let Some(station) = find_parent_station(stack) {
        if let Some(ware_id) = attrs.get("ware") {
            if let (Some(price_str), Some(amount_str)) = (attrs.get("price"), attrs.get("amount")) {
                if let (Ok(price_raw), Ok(amount)) = (price_str.parse::<i32>(), amount_str.parse::<i32>()) {
                    let price = price_raw as f64 / 100.0;

                    // Determine trade type - CORRECTED LOGIC
                    // buyer attribute means station is BUYING
                    // seller attribute means station is SELLING
                    let trade_type = if attrs.contains_key("buyer") {
                        TradeType::Buy
                    } else if attrs.contains_key("seller") {
                        TradeType::Sell
                    } else {
                        // Fallback: positive amount = selling
                        if amount > 0 { TradeType::Sell } else { TradeType::Buy }
                    };

                    // Find the sector by walking back up the stack
                    if let Some(sector_code) = find_parent_sector(stack) {
                        if price > 0.0 && amount > 0 {
                            // Get station info (we already verified station exists)
                            let station_name = get_station_name(station, component_names, localization);
                            let station_code = station.attrs.get("code")
                                .cloned()
                                .unwrap_or_else(|| station_name.clone());

                            // Get sector name
                            let sector_name = get_sector_name(&sector_code, stack, sector_names);

                            // Get sector owner
                            let sector_owner = sector_owners.get(&sector_code).and_then(|o| o.clone());

                            let ware = StationWare {
                                ware: ware_id.clone(),
                                price,
                                amount,
                                trade_type,
                                station_name,
                                station_code,
                                sector_code: sector_code.clone(),
                                sector_name,
                                sector_owner,
                            };

                            trades_by_sector
                                .entry(sector_code)
                                .or_insert_with(Vec::new)
                                .push(ware);
                        }
                    }
                }
            }
        }
    }
    // Skip trade elements not inside stations (ship orders, buildstorage, etc.)
}
