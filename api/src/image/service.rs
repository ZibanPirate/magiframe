use super::{_utils::lines::extract_lines_from_image, error::ImageServiceError};
use crate::{
    ai::service::AIService,
    image::_utils::r#const::{CONTOUR_SIZE, IMAGE_DIMENSION},
};
use edge_detection::canny;
use image::{
    imageops::colorops::dither, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba,
    RgbaImage,
};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
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

    pub async fn redraw_image_in_gif(
        &self,
        original_image: &DynamicImage,
    ) -> Result<(), ImageServiceError> {
        let _ = std::fs::create_dir("../ignore");
        original_image
            .save("../ignore/0-original.png")
            .map_err(|_| ImageServiceError::InternalError)?;

        let detection = canny(original_image.clone(), 3.0, 0.08, 0.05);
        let first_detection = detection.as_image();
        first_detection
            .save("../ignore/2-detection.png")
            .map_err(|_| ImageServiceError::InternalError)?;
        let mut lines_detection =
            DynamicImage::new_luma8(original_image.width(), original_image.height());

        for (x, y, pixel) in first_detection.pixels() {
            if pixel[0] == 0 {
                lines_detection.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }
        lines_detection
            .save("../ignore/3-lines_detection.png")
            .map_err(|_| ImageServiceError::InternalError)?;

        let mut frames: Vec<RgbaImage> = vec![];

        let lines = extract_lines_from_image(&lines_detection);
        println!("lines found: {}", lines.len());

        let one_big_line: Vec<(u32, u32)> = lines.iter().flatten().copied().collect();

        const DURATION_IN_MS: usize = 10_000;
        const FRAME_PER_SECOND: usize = 30;
        const FRAMES: usize = DURATION_IN_MS / FRAME_PER_SECOND;

        let points_per_frame = one_big_line.len() / FRAMES;

        for i in 0..FRAMES {
            let mut line_image = match frames.last() {
                Some(frame) => DynamicImage::ImageRgba8(frame.clone()),
                None => {
                    let pixels = vec![255u8; (IMAGE_DIMENSION.0 * IMAGE_DIMENSION.1 * 3) as usize];

                    let image = ImageBuffer::from_raw(IMAGE_DIMENSION.0, IMAGE_DIMENSION.1, pixels)
                        .ok_or(ImageServiceError::InternalError)?;

                    DynamicImage::ImageRgb8(image)
                }
            };

            let start = (i * points_per_frame) as usize;
            let end = ((i + 1) * points_per_frame) as usize;

            let line = &one_big_line[start..end];

            line.iter().for_each(|(x, y)| {
                let x = x.saturating_sub(CONTOUR_SIZE / 2);
                let y = y.saturating_sub(CONTOUR_SIZE / 2);

                for i in 0..CONTOUR_SIZE {
                    for j in 0..CONTOUR_SIZE {
                        let current_x = x + i;
                        let current_y = y + j;
                        if current_x >= IMAGE_DIMENSION.0 || current_y >= IMAGE_DIMENSION.1 {
                            continue;
                        }
                        line_image.put_pixel(
                            current_x,
                            current_y,
                            original_image.get_pixel(current_x, current_y),
                            // Rgba([0, 0, 0, 255]),
                        );
                    }
                }
            });

            frames.push(line_image.to_rgba8());

            // print progress every second
            if i % (FRAME_PER_SECOND) == 0 {
                println!("drawing {}%", i / (FRAMES / 100));
            }
        }

        println!("generating fade in");

        let last_frame = frames.last().unwrap().clone();

        // fade in original image
        const FADE_IN_FRAMES: usize = FRAME_PER_SECOND;
        let extra_frames = (0..FADE_IN_FRAMES)
            .into_par_iter()
            .map(|i| {
                let mut image = original_image.clone();
                for (x, y, pixel) in original_image.pixels() {
                    let blend_factor = i as f32 / FADE_IN_FRAMES as f32;
                    // blend last frame with original image
                    let last_frame_pixel = last_frame.get_pixel(x, y);
                    let new_pixel = Rgba([
                        (pixel[0] as f32 * blend_factor
                            + last_frame_pixel[0] as f32 * (1.0 - blend_factor))
                            as u8,
                        (pixel[1] as f32 * blend_factor
                            + last_frame_pixel[1] as f32 * (1.0 - blend_factor))
                            as u8,
                        (pixel[2] as f32 * blend_factor
                            + last_frame_pixel[2] as f32 * (1.0 - blend_factor))
                            as u8,
                        255,
                    ]);
                    image.put_pixel(x, y, new_pixel);
                }

                image.to_rgba8()
            })
            .collect::<Vec<RgbaImage>>();

        // add extra frames to the end
        frames.extend(extra_frames);

        frames.par_iter().enumerate().for_each(|(i, frame)| {
            frame
                .save(format!("../ignore/ani/{}.png", i))
                .map_err(|_| ImageServiceError::InternalError)
                .unwrap();
        });

        println!("frames created ✅");

        Ok(())
    }
}
