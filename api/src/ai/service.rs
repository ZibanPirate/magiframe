use super::error::AIError;
use crate::config::service::ConfigService;
use edge_detection::canny;
use image::{imageops::colorops::dither, DynamicImage, GenericImage, GenericImageView, Rgba};
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

    pub async fn generate_dithed_image(
        &self,
        query: String,
    ) -> Result<image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>, AIError> {
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
            Err(_) => return Err(AIError::InternalError),
        };

        let ai_response_string = match ai_response_string.text().await {
            Ok(ai_response_string) => ai_response_string,
            Err(_) => return Err(AIError::InternalError),
        };

        let ai_response: AIResponse = match serde_json::from_str::<AIResponse>(&ai_response_string)
        {
            Ok(ai_response) => ai_response,
            Err(_) => {
                println!("{}", ai_response_string);
                return Err(AIError::InternalError);
            }
        };

        let url = match ai_response.data.first() {
            Some(data) => data.url.clone(),
            None => return Err(AIError::InternalError),
        };

        let original_image_bytes_cursor = match reqwest::get(&url).await {
            Ok(original_image_bytes) => match original_image_bytes.bytes().await {
                Ok(original_image_bytes) => Cursor::new(original_image_bytes),
                Err(_) => return Err(AIError::InternalError),
            },
            Err(_) => return Err(AIError::InternalError),
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
            Err(_) => return Err(AIError::InternalError),
        };
        let x = (original_image.width() - cropped_width) / 2;
        let y = (original_image.height() - cropped_height) / 2;
        let original_image = original_image.crop_imm(x, y, cropped_width, cropped_height);
        let _ = std::fs::create_dir("../ignore");
        let _ = original_image.save("../ignore/0-original.png");

        let grayscale_image = original_image.to_luma8();
        let _ = grayscale_image.save("../ignore/1-grayscale.png");

        let detection = canny(grayscale_image.clone(), 3.0, 0.08, 0.05);
        let first_detection = detection.as_image();
        let _ = first_detection.save("../ignore/2-detection.png");
        let mut lines_detection =
            DynamicImage::new_luma8(grayscale_image.width(), grayscale_image.height());

        for (x, y, pixel) in first_detection.pixels() {
            if pixel[0] == 0 {
                lines_detection.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }
        let _ = lines_detection.save("../ignore/3-lines_detection.png");

        let mut dither_image = grayscale_image.clone();
        dither(&mut dither_image, &image::imageops::colorops::BiLevel);
        let _ = dither_image.save("../ignore/4-dither.png");

        Ok(dither_image)
    }
}
