use axum::{extract::{Extension, Path}, response::IntoResponse};
use crate::routes;
use crate::state::SharedState;

pub async fn start_session_handler(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    routes::start_session(state).await
}

pub async fn stop_session_handler(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    routes::stop_session(state).await
}

pub async fn get_status_handler(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    routes::get_status(state).await
}

pub async fn list_sessions_handler(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    routes::list_sessions(state).await
}

pub async fn delete_session_handler(
    Path(id): Path<i64>,
    Extension(state): Extension<SharedState>,
) -> impl IntoResponse {
    routes::delete_session(id, state).await
}

// Root handler
pub async fn root() -> &'static str {
    "Hello, Time Tracker!"
}