use crate::{_utils::error::BootError, ai::service::AIService, config::service::ConfigService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config_service: Arc<ConfigService>,
    pub ai_service: Arc<AIService>,
}

pub async fn create_app_state() -> Result<AppState, BootError> {
    let config_service = Arc::new(ConfigService::new());
    let ai_service = Arc::new(AIService::new(Arc::clone(&config_service)));

    Ok(AppState {
        config_service: Arc::clone(&config_service),
        ai_service: Arc::clone(&ai_service),
    })
}
