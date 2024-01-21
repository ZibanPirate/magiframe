pub struct Config {
    pub ai_service_auth_token: String,
}

pub struct ConfigService {}

impl ConfigService {
    pub fn new() -> Self {
        Self {}
    }

    // @TODO-ZM: memoize get_config
    pub fn get_config(&self) -> Config {
        // Load the .env file
        dotenv::dotenv().ok();

        Config {
            ai_service_auth_token: std::env::var("AI_SERVICE_AUTH_TOKEN")
                .expect("AI_SERVICE_AUTH_TOKEN env variable is missing!"),
        }
    }
}
