use crate::animations::back_forth_animation::back_forth_animation::BackAndForthAnimationState;
use iced::widget::image;
use std::{
    env,
    fs::{read, read_dir},
    path::{Path, PathBuf},
};

pub fn get_penguin_image(animation_state: BackAndForthAnimationState) -> Vec<image::Handle> {
    let paths = get_sorted_animation_paths(animation_state);

    let mut iced_image_handle = Vec::with_capacity(paths.len());
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

fn get_animation_folder(state: &BackAndForthAnimationState) -> &'static str {
    match state {
        BackAndForthAnimationState::RightAnimation => "Right Animation",
        BackAndForthAnimationState::RightToFront | BackAndForthAnimationState::LeftToFront => {
            "Front to Right Animation"
        }
        BackAndForthAnimationState::LeftAnimation => "Left Animation",
        BackAndForthAnimationState::FrontToLeft | BackAndForthAnimationState::FrontToRight => {
            "Front to Left Animation"
        }
        _ => "Front to Right Animation",
    }
}

fn should_reverse_paths(state: &BackAndForthAnimationState) -> bool {
    matches!(
        state,
        BackAndForthAnimationState::RightToFront | BackAndForthAnimationState::LeftToFront
    )
}

fn get_sorted_animation_paths(state: BackAndForthAnimationState) -> Vec<PathBuf> {
    let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let folder = get_animation_folder(&state);
    let dir_path = root
        .join("assets")
        .join("Back-Forth Animation")
        .join(folder);

    let mut paths = read_animation_directory(&dir_path);
    paths.sort();

    if should_reverse_paths(&state) {
        paths.reverse();
    }

    paths
}

fn read_animation_directory(dir_path: &Path) -> Vec<PathBuf> {
    match read_dir(dir_path) {
        Ok(entries) => {
            let mut paths = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    paths.push(entry.path());
                }
            }
            paths
        }
        Err(e) => {
            eprintln!("Failed to read directory {:?}: {}", dir_path, e);
            Vec::new()
        }
    }
}
