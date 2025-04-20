use iced::widget::image;
use std::fs::{read, read_dir};

use crate::penguin::AnimationState;

pub fn get_penguin_image(animation_state: AnimationState) -> Vec<image::Handle> {
    let mut iced_image_handle: Vec<image::Handle> = Vec::new();

    let mut paths = Vec::new();

    match animation_state {
        AnimationState::RightAnimation => {
            for asset in read_dir("assets/Right Animation").unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
            }
        }
        AnimationState::RightToFront => {
            for asset in read_dir("assets/Front to Right Animation").unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
                paths.reverse();
            }
        }
        AnimationState::LeftAnimation => {
            for asset in read_dir("assets/Left Animation").unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
            }
        }
        AnimationState::FrontToLeft => {
            for asset in read_dir("assets/Front to Left Animation").unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
            }
        }

        AnimationState::LeftToFront => {
            for asset in read_dir("assets/Front to Left Animation").unwrap() {
                let asset_path = asset.unwrap().path();
                paths.push(asset_path);
                paths.sort();
                paths.reverse();
            }
        }

        AnimationState::FrontToRight => {
            for asset in read_dir("assets/Front to Right Animation").unwrap() {
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
