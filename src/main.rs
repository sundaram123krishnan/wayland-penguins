mod assets;
mod penguin;
mod screen;
mod widgets;

use iced_layershell::{
    settings::{LayerShellSettings, Settings},
    Application,
};
use penguin::AnimatePenguin;
use screen::get_screen_dimensions;

fn main() {
    let (width, height) = get_screen_dimensions().expect("Failed to get screen dimensions");

    AnimatePenguin::run(Settings {
        flags: (width, height),
        layer_settings: LayerShellSettings {
            size: Some((width, height)),
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
}
