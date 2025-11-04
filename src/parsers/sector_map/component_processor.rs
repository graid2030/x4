use crate::models::{MapAsteroid, MapGate, MapResource, MapShip, MapStation, Position};
use std::collections::HashMap;

use super::helpers::get_station_name;
use super::types::StackItem;

#[allow(clippy::too_many_arguments)]
pub fn process_component_end(
    stack: &mut Vec<StackItem>,
    sector_code: &str,
    component_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
    stations: &mut Vec<MapStation>,
    ships: &mut Vec<MapShip>,
    gates: &mut Vec<MapGate>,
    asteroids: &mut Vec<MapAsteroid>,
    resources: &mut Vec<MapResource>,
    in_target_sector: &mut bool,
) {
    // Find matching component in stack
    let mut component_idx = None;
    for (i, item) in stack.iter().enumerate().rev() {
        if item.tag == "component" {
            component_idx = Some(i);
            break;
        }
    }

    if let Some(idx) = component_idx {
        let comp = stack.remove(idx);

        // Check if this is the sector itself - if so, exit
        if comp.attrs.get("class") == Some(&"sector".to_string())
            && comp.attrs.get("code") == Some(&sector_code.to_string())
        {
            eprintln!(
                "[SECTOR_MAP] Exiting sector. Found: {} stations, {} ships, {} gates, {} asteroids, {} resources",
                stations.len(), ships.len(), gates.len(), asteroids.len(), resources.len()
            );
            *in_target_sector = false;
            return; // Exit when leaving target sector
        }

        // Get direct parent - should be a connection element
        let connection_type = if idx > 0 && stack[idx - 1].tag == "connection" {
            stack[idx - 1].attrs.get("connection").cloned()
        } else {
            None
        };

        let position = comp.position.unwrap_or(Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let code = comp.attrs.get("code").cloned().unwrap_or_default();
        let class = comp.attrs.get("class").cloned().unwrap_or_default();
        let macro_name = comp.attrs.get("macro").cloned().unwrap_or_default();
        let owner = comp.attrs.get("owner").cloned();
        let state = comp.attrs.get("state").cloned();

        eprintln!(
            "[SECTOR_MAP] Processing component: class={}, connection_type={:?}, macro={}",
            class, connection_type, macro_name
        );

        // Skip wrecks
        if state.as_deref() == Some("wreck") {
            return;
        }

        match connection_type.as_deref() {
            Some("stations") if class == "station" => {
                let name = get_station_name(&comp.attrs, component_names, localization);
                stations.push(MapStation {
                    name,
                    code,
                    owner,
                    position,
                    macro_name,
                });
            }
            Some("ships") if class == "ship" => {
                let name = comp
                    .attrs
                    .get("name")
                    .cloned()
                    .unwrap_or_else(|| code.clone());
                ships.push(MapShip {
                    name,
                    code,
                    owner,
                    position,
                    class: macro_name.clone(),
                    macro_name,
                });
            }
            _ if class == "gate" => {
                let name = get_station_name(&comp.attrs, component_names, localization);
                let destination = comp.attrs.get("destination").cloned();
                gates.push(MapGate {
                    name,
                    code,
                    position,
                    destination,
                });
            }
            _ if macro_name.contains("asteroid") => {
                asteroids.push(MapAsteroid {
                    code,
                    position,
                    macro_name,
                });
            }
            _ if macro_name.contains("resource")
                || macro_name.contains("gas")
                || macro_name.contains("nebula") =>
            {
                let name = get_station_name(&comp.attrs, component_names, localization);
                resources.push(MapResource {
                    name,
                    code,
                    position,
                    resource_type: class,
                    macro_name,
                });
            }
            _ => {}
        }
    }
}
