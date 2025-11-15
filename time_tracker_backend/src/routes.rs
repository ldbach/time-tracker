use crate::state::{Session, SharedState};
use axum::{http::StatusCode, Json};
use chrono::{Utc, DateTime};
use serde::Serialize;
use axum::{response::IntoResponse, extract::State};
use sqlx::Row;

#[derive(Serialize)]
pub struct StatusResponse {
    pub running: bool,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
}

#[derive(Serialize)]
pub struct SessionResponse {
    pub id: i64,
    pub start_time: String,
    pub end_time: String,
    pub duration_seconds: i64,
}

// Start session
pub async fn start_session(state: SharedState) -> (StatusCode, Json<StatusResponse>) {
    let mut s = state.lock().await;
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
    let mut s = state.lock().await;
    let end = Utc::now();

    let duration = s.start_time.map(|start| (end - start).num_seconds());

    if let Some(start) = s.start_time {
        let duration_val = duration.unwrap_or(0);

        // Push to in-memory session list
        s.sessions.push(Session {
            start,
            end,
            duration: duration_val,
        });

        // Save to database
        let _ = sqlx::query(
            "INSERT INTO sessions (start_time, end_time, duration_seconds) VALUES (?1, ?2, ?3)"
        )
        .bind(start.to_rfc3339())
        .bind(end.to_rfc3339())
        .bind(duration_val)
        .execute(&s.db)
        .await;
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
    let mut s = state.lock().await;
    let duration = s.start_time.map(|start| (Utc::now() - start).num_seconds());

    Json(StatusResponse {
        running: s.running,
        start_time: s.start_time,
        end_time: None,
        duration_seconds: duration,
    })
}

// List sessions from DB
pub async fn list_sessions(state: SharedState) -> impl IntoResponse {
    let s = state.lock().await;

    let rows = sqlx::query(
        r#"SELECT id, start_time, end_time, duration_seconds
           FROM sessions
           ORDER BY id DESC"#
    )
    .fetch_all(&s.db)
    .await
    .unwrap();

    // Convert SqliteRow → SessionResponse
    let sessions: Vec<SessionResponse> = rows
        .into_iter()
        .map(|row| SessionResponse {
            id: row.get::<i64, _>("id"),
            start_time: row.get::<String, _>("start_time"),
            end_time: row.get::<String, _>("end_time"),
            duration_seconds: row.get::<i64, _>("duration_seconds"),
        })
        .collect();

    Json(sessions)
}