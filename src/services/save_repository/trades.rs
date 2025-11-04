use std::collections::{HashMap, HashSet};

use axum::http::StatusCode;

use crate::models::StationWare;

use super::SaveDataRepository;

impl SaveDataRepository {
    pub async fn get_trades(
        &self,
        sectors: Option<&[String]>,
    ) -> Result<HashMap<String, Vec<StationWare>>, StatusCode> {
        self.refresh_if_needed().await?;
        let guard = self.save_data.read().await;
        let save = guard.as_ref().ok_or(StatusCode::BAD_REQUEST)?;

        if let Some(sector_filter) = sectors {
            let filter: HashSet<&str> = sector_filter.iter().map(|s| s.as_str()).collect();
            let filtered = save
                .trades_by_sector
                .iter()
                .filter(|(code, _)| filter.contains(code.as_str()))
                .map(|(code, trades)| (code.clone(), trades.clone()))
                .collect();
            Ok(filtered)
        } else {
            Ok(save.trades_by_sector.clone())
        }
    }

    pub async fn get_all_trades(&self) -> Result<HashMap<String, Vec<StationWare>>, StatusCode> {
        self.get_trades(None).await
    }
}
