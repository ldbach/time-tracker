use axum::{extract::{Extension, Path}, response::IntoResponse};
use crate::routes;
use crate::state::SharedState;
use axum::response::Html;

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
pub async fn root() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Time Tracker API Test</title>
        <style>
            body { font-family: sans-serif; padding: 20px; }
            button { margin: 5px 0; padding: 10px 15px; display: block; }
            pre { background: #f0f0f0; padding: 10px; white-space: pre-wrap; margin-bottom: 15px; }
        </style>
    </head>
    <body>
        <h1>Time Tracker API Test</h1>

        <div>
            <button onclick="callApi('/start', 'POST', 'start-response')">Start Session</button>
            <pre id="start-response"></pre>
        </div>

        <div>
            <button onclick="callApi('/stop', 'POST', 'stop-response')">Stop Session</button>
            <pre id="stop-response"></pre>
        </div>

        <div>
            <button onclick="callApi('/status', 'GET', 'status-response')">Get Status</button>
            <pre id="status-response"></pre>
        </div>

        <div>
            <button onclick="callApi('/sessions', 'GET', 'list-response')">List All Sessions</button>
            <pre id="list-response"></pre>
        </div>

        <div>
            <button onclick="callApi('/sessions/1', 'DELETE', 'delete-response')">Delete First Session</button>
            <pre id="delete-response"></pre>
        </div>

    <script>
    async function callApi(path, method, preId) {
        const pre = document.getElementById(preId);

        try {
            const res = await fetch(path, { method });
            const data = await res.json();
            pre.textContent = JSON.stringify(data, null, 2);
        } catch(err) {
            pre.textContent = 'Error: ' + err;
        }
    }
    </script>

    </body>
    </html>
        "#)
}