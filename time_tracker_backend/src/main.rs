mod state;
mod routes;

use axum::{routing::{get, post}, Router};
use state::{SessionState, SharedState};
use routes::{start_session, stop_session, get_status};
use std::sync::{Arc, Mutex};
use axum::{
    middleware::Next,
    http::{Request, Response, header},
};
use axum::body::Body;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(SessionState {
        running: false,
        start_time: None,
        sessions: vec![],
    }));

    let app = Router::new()
        .route("/", get(root))
        .route("/start", post({
            let state = state.clone();
            move || start_session(state.clone())
        }))
        .route("/stop", post({
            let state = state.clone();
            move || stop_session(state.clone())
        }))
        .route("/status", get({
            let state = state.clone();
            move || get_status(state.clone())
        }))
        .layer(axum::middleware::from_fn(cors_middleware));

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
    // Pass the request to the next layer/handler
    let mut response = next.run(req).await;

    // Add CORS headers so the browser allows access
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        "*".parse().unwrap(),
    );
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        "GET,POST,OPTIONS".parse().unwrap(),
    );
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        "*".parse().unwrap(),
    );

    // Return the modified response
    response
}
