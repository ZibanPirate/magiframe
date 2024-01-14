use super::state::AppState;
use crate::_utils::error::BootError;
use axum::Router;

pub async fn create_app(app_state: AppState) -> Result<Router, BootError> {
    let app = Router::new();

    let app = app.with_state(app_state);

    Ok(app)
}
