mod state;
mod routes;

use axum::{routing::{get, post}, Router};
use state::{SessionState, SharedState};
use routes::{start_session, stop_session, get_status};
use std::sync::Arc;
use axum::{
    middleware::Next,
    http::{Request, Response, header},
};
use axum::body::Body;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use sqlx::ConnectOptions;
use std::str::FromStr;
use axum::extract::Extension;
use tokio::sync::Mutex;
use axum::routing::delete; // for DELETE routes
use axum::extract::Path;   // for extracting path parameter
use axum::http::Method;

#[tokio::main]
async fn main() {
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

    let state: SharedState = Arc::new(Mutex::new(SessionState {
        running: false,
        start_time: None,
        sessions: vec![],
        db, // store the pool in the state
    }));

    let app = Router::new()
    .route("/", get(root))
    .route("/start", post(start_session_handler))
    .route("/stop", post(stop_session_handler))
    .route("/status", get(get_status_handler))
    .route("/sessions", get(list_sessions_handler))
    .route("/sessions/:id", delete(delete_session_handler))
    .layer(axum::middleware::from_fn(cors_middleware))
    .layer(Extension(state.clone()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    println!("Time Tracker backend running at http://127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

// Root handler
async fn root() -> &'static str {
    "Hello, Time Tracker!"
}

// Simple CORS middleware compatible with Axum 0.7
async fn cors_middleware(req: Request<Body>, next: Next) -> Response<Body> {
    if req.method() == Method::OPTIONS {
        // Respond directly to preflight requests
        let mut response = Response::new(Body::empty());
        let headers = response.headers_mut();
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
        headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "GET,POST,DELETE,OPTIONS".parse().unwrap());
        headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
        return response;
    }

    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "GET,POST,DELETE,OPTIONS".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
    response
}

// Async handlers
async fn stop_session_handler(Extension(state): Extension<SharedState>) -> impl axum::response::IntoResponse {
    routes::stop_session(state).await
}

async fn start_session_handler(Extension(state): Extension<SharedState>) -> impl axum::response::IntoResponse {
    routes::start_session(state).await
}

async fn get_status_handler(Extension(state): Extension<SharedState>) -> impl axum::response::IntoResponse {
    routes::get_status(state).await
}

async fn list_sessions_handler(Extension(state): Extension<SharedState>) -> impl axum::response::IntoResponse {
    routes::list_sessions(state).await
}

async fn delete_session_handler(
    Path(id): Path<i64>,
    Extension(state): Extension<SharedState>,
) -> impl axum::response::IntoResponse {
    routes::delete_session(id, state).await
}