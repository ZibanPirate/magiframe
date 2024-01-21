use crate::{
    _utils::error::BootError, ai::service::AIService, config::service::ConfigService,
    image::service::ImageService,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config_service: Arc<ConfigService>,
    pub ai_service: Arc<AIService>,
    pub image_service: Arc<ImageService>,
}

pub async fn create_app_state() -> Result<AppState, BootError> {
    let config_service = Arc::new(ConfigService::new());
    let ai_service = Arc::new(AIService::new(Arc::clone(&config_service)));
    let image_service = Arc::new(ImageService::new(Arc::clone(&ai_service)));

    Ok(AppState {
        config_service: Arc::clone(&config_service),
        ai_service: Arc::clone(&ai_service),
        image_service: Arc::clone(&image_service),
    })
}
