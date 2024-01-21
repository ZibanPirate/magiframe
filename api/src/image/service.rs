use super::error::ImageServiceError;
use crate::ai::service::AIService;
use edge_detection::canny;
use image::{
    codecs::gif::GifEncoder, imageops::colorops::dither, DynamicImage, Frame, GenericImage,
    GenericImageView, Rgba,
};
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
        let original_image = match image::open("../ignore/0-original.png") {
            Ok(image) => image,
            Err(_) => self
                .ai_service
                .generate_image(query)
                .await
                .map_err(|_| ImageServiceError::InternalError)?,
        };

        let grayscale_image = original_image.to_luma8();

        let mut dither_image = grayscale_image.clone();
        dither(&mut dither_image, &image::imageops::colorops::BiLevel);

        self.redraw_image_in_gif(&original_image).await?;

        Ok(dither_image)
    }

    pub async fn redraw_image_in_gif(&self, image: &DynamicImage) -> Result<(), ImageServiceError> {
        let _ = std::fs::create_dir("../ignore");
        image
            .save("../ignore/0-original.png")
            .map_err(|_| ImageServiceError::InternalError)?;

        let detection = canny(image.clone(), 3.0, 0.08, 0.05);
        let first_detection = detection.as_image();
        first_detection
            .save("../ignore/2-detection.png")
            .map_err(|_| ImageServiceError::InternalError)?;
        let mut lines_detection = DynamicImage::new_luma8(image.width(), image.height());

        for (x, y, pixel) in first_detection.pixels() {
            if pixel[0] == 0 {
                lines_detection.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }
        lines_detection
            .save("../ignore/3-lines_detection.png")
            .map_err(|_| ImageServiceError::InternalError)?;

        let frames = vec![
            image.to_rgba8(),
            first_detection.to_rgba8(),
            lines_detection.to_rgba8(),
        ];

        let gif_output = std::fs::File::create("../ignore/5-animated.gif")
            .map_err(|_| ImageServiceError::InternalError)?;
        let mut gif_encoder = GifEncoder::new(gif_output);

        for frame in frames {
            let img_frame = Frame::new(frame);
            gif_encoder
                .encode_frame(img_frame.clone())
                .map_err(|_| ImageServiceError::InternalError)?;
        }

        Ok(())
    }
}
