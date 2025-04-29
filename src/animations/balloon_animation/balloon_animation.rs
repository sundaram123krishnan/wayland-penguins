use iced::{widget::image, Task};

use crate::{animations::animation::AnimationMessage, penguin::Message};

use super::balloon_animation_assets::get_balloon_image;

pub struct BalloonAnimation {
    pub current_pos_x: f32,
    pub current_pos_y: f32,
    pub balloon_with_penguin: image::Handle,
    pub balloon_without_penguin: image::Handle,
    pub sprite_height: f32,
    pub sprite_width: f32,
}

pub enum BalloonAnimationState {
    BallonWithPenguin,
    BalloonWithoutPenguin,
}

#[derive(Debug, Clone)]
pub enum BalloonAnimationMessage {
    Tick,
}
impl BalloonAnimation {
    pub fn new(screen_size: (u32, u32)) -> Self {
        let balloon_with_penguin = get_balloon_image(BalloonAnimationState::BallonWithPenguin);
        let balloon_without_penguin =
            get_balloon_image(BalloonAnimationState::BalloonWithoutPenguin);
        Self {
            current_pos_x: 0.0,
            current_pos_y: 100.0,
            balloon_with_penguin,
            balloon_without_penguin,
            sprite_height: 180.0,
            sprite_width: 180.0,
        }
    }

    pub fn update(&mut self, message: BalloonAnimationMessage) -> Task<Message> {
        match message {
            BalloonAnimationMessage::Tick => {
                self.current_pos_y += 1.0;
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(16)).map(|_| {
            Message::PlayAnimationMessage(AnimationMessage::BalloonMessage(
                BalloonAnimationMessage::Tick,
            ))
        })
        // 1000ms / 16ms approx 60 fps
    }
}
