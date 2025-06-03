use iced::widget::image;
use std::env;
use std::fs::read;

pub fn get_penguin_copter_image() -> image::Handle {
    let root = env::current_dir().unwrap();

    let assets_dir = root.join("assets").join("PenguinCopter");
    let asset_path = assets_dir.join("pixelated_penguin_copter.png");
    let image_bytes = read(&asset_path).unwrap();
    image::Handle::from_bytes(image_bytes)
}
