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

// Current state of session
#[derive(Debug)]
pub struct SessionState {
    pub running: bool,
    pub start_time: Option<DateTime<Utc>>,
    pub sessions: Vec<Session>,
    pub db: SqlitePool,
}

// Arc: allows shared ownership across tasks
// Mutex: allows safe mutation of the shared data
pub type SharedState = Arc<Mutex<SessionState>>;