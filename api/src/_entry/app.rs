use super::state::AppState;
use crate::{_utils::error::BootError, image::controller::create_image_router};
use axum::Router;

pub async fn create_app(app_state: AppState) -> Result<Router, BootError> {
    let app = Router::new();

    let app = app
        //
        .nest("/image", create_image_router())
        .with_state(app_state);

    Ok(app)
}
