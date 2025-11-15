use std::sync::Arc;
use chrono::{DateTime, Utc};
use sqlx::sqlite::SqlitePool;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct Session {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub duration: i64,
}

#[derive(Debug)]
pub struct SessionState {
    pub running: bool,
    pub start_time: Option<DateTime<Utc>>,
    pub sessions: Vec<Session>,
    pub db: SqlitePool,
}

// SharedState is now Arc<tokio::Mutex<SessionState>>
pub type SharedState = Arc<Mutex<SessionState>>;