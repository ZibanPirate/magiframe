use crate::_utils::error::BootError;

#[derive(Clone)]
pub struct AppState {}

pub async fn create_app_state() -> Result<AppState, BootError> {
    Ok(AppState {})
}
