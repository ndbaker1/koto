use std::{env, net::SocketAddr, sync::Arc};

use axum::{
    extract::ws::{Message, WebSocket},
    response::IntoResponse,
    routing, Extension, Router,
};
use tracing::Level;
use tracing_subscriber;

use koto_core::Game;
use session::SessionManager;

mod messages;
mod session;
mod ws;

pub type SharedServerState = Arc<SessionManager<WebSocket, Message, Game>>;

#[tokio::main]
async fn main() {
    // env setup
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("8000"))
        .parse()
        .expect("PORT must be a number");

    // app state setup
    let state = SharedServerState::default();

    // axum setup
    let app = Router::new()
        .route("/api/health", routing::get(health_handler))
        .route("/api/ws", routing::get(ws::websocket_handler))
        .layer(Extension(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // logging
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::debug!("listening on {}", addr);

    // run 🏃
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Health Check Endpoint used to verify the service is live
async fn health_handler() -> impl IntoResponse {
    tracing::info!("HEALTH_CHECK ✓");
    "health check ✓".into_response()
}
