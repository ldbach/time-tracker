use crate::state::{Session, SharedState};
use axum::{http::StatusCode, Json};
use chrono::{Utc, DateTime};
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    pub running: bool,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
}

// Start session
pub async fn start_session(state: SharedState) -> (StatusCode, Json<StatusResponse>) {
    let mut s = state.lock().unwrap();
    s.running = true;
    s.start_time = Some(Utc::now());

    (
        StatusCode::OK,
        Json(StatusResponse {
            running: s.running,
            start_time: s.start_time,
            end_time: None,
            duration_seconds: None,
        }),
    )
}

// Stop session
pub async fn stop_session(state: SharedState) -> (StatusCode, Json<StatusResponse>) {
    let mut s = state.lock().unwrap();
    let end = Utc::now();

    let duration = s.start_time.map(|start| (end - start).num_seconds());

    if let Some(start) = s.start_time {
        s.sessions.push(Session {
            start,
            end,
            duration: duration.unwrap_or(0),
        });
    }

    s.running = false;
    s.start_time = None;

    (
        StatusCode::OK,
        Json(StatusResponse {
            running: s.running,
            start_time: None,
            end_time: Some(end),
            duration_seconds: duration,
        }),
    )
}

// Get current status
pub async fn get_status(state: SharedState) -> Json<StatusResponse> {
    let s = state.lock().unwrap();
    let duration = s.start_time.map(|start| (Utc::now() - start).num_seconds());

    Json(StatusResponse {
        running: s.running,
        start_time: s.start_time,
        end_time: None,
        duration_seconds: duration,
    })
}