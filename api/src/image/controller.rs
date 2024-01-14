use axum::{response::IntoResponse, Json, Router};
use serde_json::json;

use crate::_entry::state::AppState;

pub async fn random() -> impl IntoResponse {
    Json(json!({
        "image here": "random"
    }))
    .into_response()
}

pub fn create_image_router() -> Router<AppState> {
    Router::new().route("/random", axum::routing::get(random))
}
