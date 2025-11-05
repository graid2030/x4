use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

use super::handlers::{get_last_paths, get_sectors, get_sectors_list, get_sector_map, get_status, get_trade_offers, get_ware_trades, get_wares, get_pilot, get_player_property, init_handler, list_save_files, get_dashboard, AppState};
use super::sector_detail::get_sector_detail;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/init", post(init_handler))
        .route("/api/dashboard", get(get_dashboard))
        .route("/api/sectors/list", get(get_sectors_list))
        .route("/api/sectors/:code", get(get_sector_detail))
        .route("/api/sectors", get(get_sectors))
        .route("/api/wares", get(get_wares))
        .route("/api/last-paths", get(get_last_paths))
        .route("/api/list-saves", post(list_save_files))
        .route("/api/status", get(get_status))
        .route("/api/pilot", get(get_pilot))
        .route("/api/player-property", get(get_player_property))
        .route("/api/trade-offers", post(get_trade_offers))
        .route("/api/ware-trades", post(get_ware_trades))
        .route("/api/sector-map", post(get_sector_map))
        .nest_service(
            "/",
            ServeDir::new("static").fallback(ServeFile::new("static/index.html"))
        )
        .with_state(state)
}
