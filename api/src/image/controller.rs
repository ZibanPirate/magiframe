use crate::_entry::state::AppState;
use axum::{extract::State, response::IntoResponse, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use hyper::StatusCode;
use image::ImageOutputFormat;
use std::io::Cursor;

pub async fn random(State(app_state): State<AppState>) -> impl IntoResponse {
    // "a lone flower in hill looking down on a farm with animals, with light background"
    // "angry but cute elephant, with light background"
    // "flat 2d arabic and japaneses calligraphy art, with a light background"
    // "illustration of lone farm house in a hill with a tree beside it, and a light background"
    // "flat 2d maldives calligraphy art, with a light background"
    // "anime style, luffy vs doflamingo from one-piece, with light background"
    // "random cute cartoonish art, with light background"
    let image = match app_state
        .image_service
        .generate_dithed_image(
            "random image, with light background which i can use for my wall frame".to_string(),
        )
        .await
    {
        Ok(image) => image,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let mut buffer = Cursor::new(Vec::new());
    if (image.write_to(&mut buffer, ImageOutputFormat::Png)).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let bytes: Vec<u8> = buffer.into_inner();

    (TypedHeader(ContentType::png()), bytes).into_response()
}

pub fn create_image_router() -> Router<AppState> {
    Router::new().route("/random", axum::routing::get(random))
}
