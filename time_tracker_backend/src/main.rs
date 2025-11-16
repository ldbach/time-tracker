mod state;
mod routes;
mod handlers;
mod middleware;

use axum::{routing::{get, post, delete}, Router, extract::Extension};
use state::{SessionState, SharedState};
use handlers::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;
use std::env;

#[tokio::main]
async fn main() {
    // Database setup
    // Make sure folder "data" exists before running
    std::fs::create_dir_all("data").expect("Failed to create data folder");

    let opts = SqliteConnectOptions::from_str("sqlite:data/sessions.db")
        .expect("Failed to parse database options")
        .create_if_missing(true);

    let db = SqlitePool::connect_with(opts)
    .await
    .expect("Failed to connect to SQLite");

    // Create the sessions table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            duration_seconds INTEGER NOT NULL
        )
        "#
    )
    .execute(&db)
    .await
    .expect("Failed to create sessions table");
    // End of the Database setup

    let state: SharedState = Arc::new(Mutex::new(SessionState {
        running: false,
        start_time: None,
        sessions: vec![],
        db,
    }));

    let app = Router::new()
    .route("/", get(root))
    .route("/start", post(start_session_handler))
    .route("/stop", post(stop_session_handler))
    .route("/status", get(get_status_handler))
    .route("/sessions", get(list_sessions_handler))
    .route("/sessions/:id", delete(delete_session_handler))
    .layer(axum::middleware::from_fn(middleware::cors_middleware)) // Adds a CORS middleware to allow requests from browsers
    .layer(Extension(state.clone())); // passes shared state into all handlers

    // Get the Render-assigned port from environment or fallback to 3001 locally
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = format!("0.0.0.0:{}", port); // listen on all network interfaces (local + LAN + public IP)
    let display_addr = format!("http://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("Time Tracker backend running at {}", display_addr);
    axum::serve(listener, app).await.unwrap();
}