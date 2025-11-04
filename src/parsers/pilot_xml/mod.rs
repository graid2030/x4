mod state;

use anyhow::Result;
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;

use crate::models::PilotInfo;

use state::PilotState;

/// Extract basic pilot info (player name and credits) from save
pub fn extract_pilot_info(
    xml_content: &str,
    localization: &HashMap<String, HashMap<String, String>>,
) -> Result<PilotInfo> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut state = PilotState::new(localization);

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => state.handle_start(e),
            Ok(Event::Empty(ref e)) => state.handle_empty(e),
            Ok(Event::End(_)) => state.handle_end(),
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow::anyhow!("XML parse error: {}", e)),
            _ => {}
        }

        if state.is_complete() {
            break;
        }

        buf.clear();
    }

    Ok(state.into_info())
}
