mod fallback;
mod parser;

use anyhow::Result;
use std::collections::HashMap;

use crate::models::Sector;

pub fn extract_sectors(
    xml_content: &str,
    sector_names: &HashMap<String, String>,
    component_names: &HashMap<String, String>,
    sector_code_names: &HashMap<String, String>,
    localization: &HashMap<String, HashMap<String, String>>,
) -> Result<Vec<Sector>> {
    let mut sectors = parser::parse_sectors(
        xml_content,
        sector_names,
        component_names,
        sector_code_names,
        localization,
    )?;

    if sectors.is_empty() {
        sectors = fallback::fallback_scan(
            xml_content,
            sector_names,
            component_names,
            sector_code_names,
        );
    }

    sectors.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(sectors)
}
