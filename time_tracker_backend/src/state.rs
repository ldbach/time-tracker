use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

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
}

pub type SharedState = Arc<Mutex<SessionState>>;