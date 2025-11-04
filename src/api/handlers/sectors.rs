use axum::{extract::State, http::StatusCode, Json};
use std::collections::HashMap;

use crate::models::{Sector, SectorsListResponse, SectorListItem, SectorStats};
use crate::parsers::save_xml::{extract_all_trades, load_save_file};

use super::common::AppState;

/// Get available sectors
pub async fn get_sectors(State(state): State<AppState>) -> Result<Json<Vec<Sector>>, StatusCode> {
    let save_data = state.save_data.read().await;
    match save_data.as_ref() {
        Some(data) => Ok(Json(data.sectors.clone())),
        None => Err(StatusCode::BAD_REQUEST),
    }
}

/// Get sectors list with aggregated statistics
pub async fn get_sectors_list(
    State(state): State<AppState>,
) -> Result<Json<SectorsListResponse>, StatusCode> {
    let game_data = state.game_data.read().await;
    let save_data = state.save_data.read().await;

    let (game, save) = match (game_data.as_ref(), save_data.as_ref()) {
        (Some(g), Some(s)) => (g, s),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Load save file for station/trade counts
    let save_content = load_save_file(&save.save_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract all trades to count offers
    let all_trades = extract_all_trades(
        &save_content,
        game.sector_names_map(),
        game.component_names_map(),
        game.localization_map(),
        &save.sectors,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Count stations per sector
    let station_counts = count_stations_per_sector(&save_content)?;

    // Build sector list items
    let mut sectors_list: Vec<SectorListItem> = Vec::new();
    let mut total_stations = 0;
    let mut player_owned_sectors = 0;
    let mut discovered_count = 0;

    for sector in &save.sectors {
        let sector_code = &sector.code;

        // Count offers by type
        let (buy_offers, sell_offers) = if let Some(trades) = all_trades.get(sector_code) {
            let buys = trades.iter().filter(|t| matches!(t.trade_type, crate::models::TradeType::Buy)).count();
            let sells = trades.iter().filter(|t| matches!(t.trade_type, crate::models::TradeType::Sell)).count();
            (buys, sells)
        } else {
            (0, 0)
        };

        // Get station counts
        let (station_count, player_stations) = station_counts
            .get(sector_code)
            .copied()
            .unwrap_or((0, 0));

        total_stations += station_count;

        // Check if player owns sector
        let is_player_owned = sector.owner.as_ref().map(|o| o == "player").unwrap_or(false);
        if is_player_owned {
            player_owned_sectors += 1;
        }

        // Discovered (for now assume all discovered, will add knownto later)
        let discovered = true;
        if discovered {
            discovered_count += 1;
        }

        // Map owner to name (fallback to raw owner ID if not in map)
        let owner_name = sector
            .owner
            .as_ref()
            .map(|o| game.get_faction_display_name(o));

        sectors_list.push(SectorListItem {
            code: sector.code.clone(),
            name: sector.name.clone(),
            macro_name: sector.macro_name.clone(),
            owner: sector.owner.clone(),
            owner_name,
            contested: false, // TODO: extract from save
            station_count,
            player_stations,
            buy_offers,
            sell_offers,
            total_trades: buy_offers + sell_offers,
            resource_types: vec![], // TODO: Phase 4 - resource areas
            discovered,
        });
    }

    // Sort by name
    sectors_list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let stats = SectorStats {
        total_sectors: save.sectors.len(),
        player_owned: player_owned_sectors,
        discovered: discovered_count,
        total_stations,
    };

    Ok(Json(SectorsListResponse {
        sectors: sectors_list,
        stats,
    }))
}

/// Count stations per sector from save XML
pub fn count_stations_per_sector(xml_content: &str) -> Result<HashMap<String, (usize, usize)>, StatusCode> {
    use quick_xml::{events::Event, Reader};

    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut station_counts: HashMap<String, (usize, usize)> = HashMap::new();
    let mut buf = Vec::new();

    // Track current sector and depth
    let mut current_sector: Option<String> = None;
    let mut sector_depth: usize = 0;
    let mut component_depth: usize = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"component" => {
                        component_depth += 1;

                        let mut is_sector = false;
                        let mut is_station = false;
                        let mut code = None;
                        let mut owner = None;

                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"class" => {
                                    let class = String::from_utf8_lossy(&attr.value);
                                    if class == "sector" {
                                        is_sector = true;
                                    } else if class == "station" {
                                        is_station = true;
                                    }
                                }
                                b"code" => code = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                b"owner" => owner = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                _ => {}
                            }
                        }

                        if is_sector {
                            current_sector = code;
                            sector_depth = component_depth;
                        } else if is_station && current_sector.is_some() {
                            if let Some(ref sector_code) = current_sector {
                                let entry = station_counts.entry(sector_code.clone()).or_insert((0, 0));
                                entry.0 += 1; // total stations
                                if owner.as_ref().map(|o| o == "player").unwrap_or(false) {
                                    entry.1 += 1; // player stations
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name().as_ref() == b"component" {
                    // If we're closing the sector component, reset current_sector
                    if component_depth == sector_depth && current_sector.is_some() {
                        current_sector = None;
                        sector_depth = 0;
                    }
                    component_depth = component_depth.saturating_sub(1);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("XML parse error: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(station_counts)
}
