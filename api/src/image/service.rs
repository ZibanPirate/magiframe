use super::error::ImageServiceError;
use crate::ai::service::AIService;
use edge_detection::canny;
use image::{imageops::colorops::dither, DynamicImage, GenericImage, GenericImageView, Rgba};
use std::sync::Arc;

pub struct ImageService {
    ai_service: Arc<AIService>,
}

impl ImageService {
    pub fn new(ai_service: Arc<AIService>) -> Self {
        Self { ai_service }
    }

    pub async fn generate_dithed_image(
        &self,
        query: String,
    ) -> Result<image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>, ImageServiceError> {
        let original_image = self
            .ai_service
            .generate_image(query)
            .await
            .map_err(|_| ImageServiceError::InternalError)?;

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
