use iced::widget::image;
use std::{
    env,
    fs::{read, read_dir},
};

use crate::animations::back_forth_animation::back_forth_animation::BackAndForthAnimationState;

pub fn get_penguin_image(animation_state: BackAndForthAnimationState) -> Vec<image::Handle> {
    let mut iced_image_handle: Vec<image::Handle> = Vec::new();

    let mut paths = Vec::new();

    let root = env::current_dir().unwrap();

    let assets_dir = root.join("assets").join("Back-Forth Animation");

    match animation_state {
        BackAndForthAnimationState::RightAnimation => {
            for asset in read_dir(assets_dir.join("Right Animation")).unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
            }
        }
        BackAndForthAnimationState::RightToFront => {
            for asset in read_dir(assets_dir.join("Front to Right Animation")).unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
                paths.reverse();
            }
        }
        BackAndForthAnimationState::LeftAnimation => {
            for asset in read_dir(assets_dir.join("Left Animation")).unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
            }
        }
        BackAndForthAnimationState::FrontToLeft => {
            for asset in read_dir(assets_dir.join("Front to Left Animation")).unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
            }
        }

        BackAndForthAnimationState::LeftToFront => {
            for asset in read_dir(assets_dir.join("Front to Left Animation")).unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
                paths.reverse();
            }
        }

        BackAndForthAnimationState::FrontToRight => {
            for asset in read_dir(assets_dir.join("Front to Right Animation")).unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
            }
        }
        _ => todo!(),
    }

    for image_path in paths {
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
    iced_image_handle
}
