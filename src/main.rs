mod animations;
mod penguin;
mod screen;
mod widgets;

use iced_layershell::{reexport::Anchor, settings::LayerShellSettings};

use iced_layershell::build_pattern::application;
use penguin::AnimatePenguin;
use screen::get_screen_dimensions;

fn main() {
    let (width, height) = get_screen_dimensions().expect("Failed to get screen dimensions");

    application(
        AnimatePenguin::namespace,
        AnimatePenguin::update,
        AnimatePenguin::view,
    )
    .style(AnimatePenguin::style)
    .subscription(AnimatePenguin::subscription)
    .layer_settings(LayerShellSettings {
        exclusive_zone: -1,
        anchor: Anchor::all(),
        margin: (0, 0, 0, 0),
        events_transparent: true,
        ..Default::default()
    })
    .run_with(move || AnimatePenguin::new((width, height)))
    .unwrap();
}
