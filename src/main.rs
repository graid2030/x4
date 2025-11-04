mod models;
mod parsers;
mod extractors;
mod services;
mod api;

use api::{create_router, AppState};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let state = AppState {
        game_data: Arc::new(RwLock::new(None)),
        save_data: Arc::new(RwLock::new(None)),
    };

    let app = create_router(state);

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    println!("X4 Trading System running on http://{}", addr);
    println!("Open this URL in your browser to get started");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
