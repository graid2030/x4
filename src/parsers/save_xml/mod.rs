mod helpers;
mod sectors;
mod station_positions;
mod stations;
mod trades;

use anyhow::Result;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub use sectors::extract_sectors;
pub use station_positions::extract_stations_for_sector;
pub use stations::{extract_station_data, StationParseResult};
pub use trades::extract_all_trades;

/// Load save file (supports both .xml and .xml.gz)
pub fn load_save_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    let mut file = File::open(path)?;
    let mut content = String::new();

    if path.to_string_lossy().ends_with(".gz") {
        let mut decoder = GzDecoder::new(file);
        decoder.read_to_string(&mut content)?;
    } else {
        file.read_to_string(&mut content)?;
    }

    Ok(content)
}
