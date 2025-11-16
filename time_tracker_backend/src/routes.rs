use crate::state::{Session, SharedState};
use axum::{http::StatusCode, Json};
use chrono::{Utc, DateTime};
use serde::Serialize;
use axum::response::IntoResponse;
use sqlx::{Row, SqlitePool};

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
    let mut s = state.lock().await; // lock the shared state
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
    let mut s = state.lock().await; // lock the shared state
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
    let s = state.lock().await; // lock the shared state
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
    let s = state.lock().await; // lock the shared state

    let rows = sqlx::query(
        r#"SELECT id, start_time, end_time, duration_seconds
           FROM sessions
           ORDER BY id DESC"#
    )
    .fetch_all(&s.db) // executes the query and returns all rows as Vec<SqliteRow>
    .await
    .unwrap();

    // Convert SqliteRow to SessionResponse
    let sessions: Vec<SessionResponse> = rows
        .into_iter() // iterate over all SqliteRows
        .map(|row| SessionResponse {
            id: row.get::<i64, _>("id"),
            start_time: row.get::<String, _>("start_time"),
            end_time: row.get::<String, _>("end_time"),
            duration_seconds: row.get::<i64, _>("duration_seconds"),
        })
        .collect(); // gather all SessionResponse structs into a Vec<SessionResponse>

    Json(sessions)
}

// Delete session
pub async fn delete_session(
    id: i64,
    state: SharedState,
) -> impl IntoResponse {
    let s = state.lock().await; // lock the shared state
    let db: &SqlitePool = &s.db;

    let result = sqlx::query("DELETE FROM sessions WHERE id = ?1")
        .bind(id)
        .execute(db)
        .await;

    match result {
        Ok(_) => (axum::http::StatusCode::OK, Json(serde_json::json!({"deleted": id}))),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))),
    }
}