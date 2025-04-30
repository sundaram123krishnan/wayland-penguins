use super::balloon_animation_assets::get_balloon_image;
use crate::{animations::animation::AnimationMessage, penguin::Message};
use iced::{widget::image, Task};

pub struct BalloonAnimation {
    pub current_pos_x: f32,
    pub current_pos_y: f32,
    pub balloon_with_penguin: image::Handle,
    pub balloon_without_penguin: image::Handle,
    pub sprite_height: f32,
    pub sprite_width: f32,
    descent_speed: f32,
    with_penguin: bool,
    landing_y: f32,
    pub landed: bool,
    hover_offset: f32,
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

        let max_x = (screen_size.0 as f32 - 200.0).max(200.0);
        let random_x = rand::random::<f32>() * max_x;
        let random_y = rand::random::<f32>() * 200.0 - 100.0;

        let landing_y = screen_size.1 as f32 - 200.0;

        Self {
            current_pos_x: random_x,
            current_pos_y: random_y,
            balloon_with_penguin,
            balloon_without_penguin,
            sprite_height: 180.0,
            sprite_width: 180.0,
            descent_speed: 0.5,
            with_penguin: true,
            landing_y,
            landed: false,
            hover_offset: 0.0,
        }
    }

    pub fn update(&mut self, message: BalloonAnimationMessage) -> Task<Message> {
        match message {
            BalloonAnimationMessage::Tick => {
                if !self.landed {
                    // have a falling effect
                    self.hover_offset = 8.0 * (self.current_pos_y * 0.02).sin();
                    self.current_pos_y += self.descent_speed;

                    let drift_speed = 1.2;
                    let drift = drift_speed * (self.current_pos_y * 0.01).cos();
                    self.current_pos_x += drift;

                    if self.current_pos_y >= self.landing_y {
                        self.current_pos_y = self.landing_y;
                        self.landed = true;
                        self.with_penguin = false;
                    }
                } else {
                    self.hover_offset = 5.0 * (self.current_pos_y * 0.04).cos();
                    self.current_pos_y = self.landing_y + self.hover_offset;
                }

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
    }
}
