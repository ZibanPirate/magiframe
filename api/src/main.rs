mod _entry;
mod _utils;
mod ai;
mod config;
mod image;

#[tokio::main]
async fn main() {
    #![allow(clippy::unwrap_used)]
    _entry::boot::boot_up().await.unwrap();
}
