use super::error::AIServiceError;
use crate::config::service::ConfigService;
use image::DynamicImage;
use serde::Deserialize;
use std::{io::Cursor, sync::Arc};

#[derive(Deserialize)]
pub struct AIResponseData {
    url: String,
}

#[derive(Deserialize)]
pub struct AIResponse {
    data: Vec<AIResponseData>,
}

pub struct AIService {
    config_service: Arc<ConfigService>,
}

impl AIService {
    pub fn new(config_service: Arc<ConfigService>) -> Self {
        Self { config_service }
    }

    pub async fn generate_image(&self, query: String) -> Result<DynamicImage, AIServiceError> {
        let ai_service_auth_token = self.config_service.get_config().ai_service_auth_token;
        let client = reqwest::Client::new();

        let ai_response_string = client
            .post("https://api.openai.com/v1/images/generations")
            .header("content-type", "application/json")
            .header("accept", "application/json")
            .header("Authorization", format!("Bearer {}", ai_service_auth_token))
            .body(format!(
                r#"{{
    "model": "dall-e-3",
    "prompt": "{}",
    "n": 1,
    "size": "1024x1024"
}}"#,
                query
            ))
            .send()
            .await;

        let ai_response_string = match ai_response_string {
            Ok(ai_response_string) => ai_response_string,
            Err(_) => return Err(AIServiceError::InternalError),
        };

        let ai_response_string = match ai_response_string.text().await {
            Ok(ai_response_string) => ai_response_string,
            Err(_) => return Err(AIServiceError::InternalError),
        };

        let ai_response: AIResponse = match serde_json::from_str::<AIResponse>(&ai_response_string)
        {
            Ok(ai_response) => ai_response,
            Err(_) => {
                println!("{}", ai_response_string);
                return Err(AIServiceError::InternalError);
            }
        };

        let url = match ai_response.data.first() {
            Some(data) => data.url.clone(),
            None => return Err(AIServiceError::InternalError),
        };

        let original_image_bytes_cursor = match reqwest::get(&url).await {
            Ok(original_image_bytes) => match original_image_bytes.bytes().await {
                Ok(original_image_bytes) => Cursor::new(original_image_bytes),
                Err(_) => return Err(AIServiceError::InternalError),
            },
            Err(_) => return Err(AIServiceError::InternalError),
        };

        let (cropped_width, cropped_height) = (480, 800);
        let max_dimension = cropped_width.max(cropped_height);
        let original_image = match image::load(original_image_bytes_cursor, image::ImageFormat::Png)
        {
            Ok(img) => img.resize(
                max_dimension,
                max_dimension,
                image::imageops::FilterType::Lanczos3,
            ),
            Err(_) => return Err(AIServiceError::InternalError),
        };

        let x = (original_image.width() - cropped_width) / 2;
        let y = (original_image.height() - cropped_height) / 2;
        let original_image = original_image.crop_imm(x, y, cropped_width, cropped_height);

        Ok(original_image)
    }
}
