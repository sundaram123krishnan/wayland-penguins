use std::{
    env,
    fs::{read, read_dir},
};

use iced::widget::image;

use super::balloon_animation::BalloonAnimationState;

pub fn get_balloon_image(balloon_animation_state: BalloonAnimationState) -> image::Handle {
    let root = env::current_dir().unwrap();

    let assets_dir = root.join("assets").join("Balloon Animation");

    let mut balloon_images = Vec::new();
    let mut iced_image_handle = Vec::new();

    for asset in read_dir(assets_dir).unwrap() {
        let asset_path = asset.unwrap().path();
        balloon_images.push(asset_path);
    }

    for image_path in balloon_images {
        match read(&image_path) {
            Ok(image_bytes) => {
                let image_handle = image::Handle::from_bytes(image_bytes);
                iced_image_handle.push(image_handle);
            }
            Err(e) => {
                eprintln!("Failed to read image {:?}: {}", image_path, e);
            }
        }
    }

    match balloon_animation_state {
        BalloonAnimationState::BallonWithPenguin => iced_image_handle[0].clone(),
        BalloonAnimationState::BalloonWithoutPenguin => iced_image_handle[1].clone(),
    }
}
