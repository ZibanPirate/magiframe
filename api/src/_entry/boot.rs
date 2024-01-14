use super::app::create_app;
use super::state::create_app_state;
use crate::_utils::error::BootError;

pub async fn boot_up() -> Result<(), BootError> {
    let app_state = create_app_state().await?;

    let app = create_app(app_state).await?;

    let address = "0.0.0.0:8080";

    let listener = match tokio::net::TcpListener::bind(address).await {
        Ok(listener) => listener,
        Err(error) => {
            return Err(BootError::Bind {
                address: address.to_string(),
                error: error.to_string(),
            })
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => Ok(()),
        Err(error) => Err(BootError::Server {
            error: error.to_string(),
        }),
    }
}
