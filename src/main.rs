mod animations;
mod penguin;
mod widgets;

use iced_layershell::{reexport::Anchor, settings::LayerShellSettings};

use iced_layershell::application;
use penguin::AnimatePenguin;

fn main() {
    application(
        AnimatePenguin::new,
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
    .run()
    .unwrap();
}
