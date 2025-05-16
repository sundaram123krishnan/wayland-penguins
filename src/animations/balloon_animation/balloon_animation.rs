use super::balloon_animation_assets::get_balloon_image;
use crate::{animations::animation::AnimationMessage, penguin::Message};
use iced::{widget::image, Task};
use rand::Rng;

pub struct BalloonAnimation {
    pub current_pos_x: f32,
    pub current_pos_y: f32,
    pub balloon_with_penguin: image::Handle,
    pub balloon_without_penguin: image::Handle,
    pub balloon_with_hyprland_logo: image::Handle,
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
    BalloonWithHyprlandLogo,
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

        let balloon_with_hyprland_logo =
            get_balloon_image(BalloonAnimationState::BalloonWithHyprlandLogo);
        let sprite_height = 180.0;
        let sprite_width = 180.0;

        let screen_x = screen_size.0;

        let left_margin = sprite_width * 1.75;

        let right_margin = screen_x as f32 - (sprite_width * 1.75);

        let mut rnd = rand::rng();

        let random_x = rnd.random_range(left_margin..right_margin);

        let landing_y = screen_size.1 as f32 - 200.0;

        Self {
            current_pos_x: random_x,
            current_pos_y: 10.0,
            balloon_with_penguin,
            balloon_without_penguin,
            balloon_with_hyprland_logo,
            sprite_height,
            sprite_width,
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
                    self.hover_offset = 5.0 * (self.current_pos_y * 0.04).sin();

                    self.current_pos_y += self.descent_speed;

                    let drift_speed = 0.8;
                    let drift = drift_speed * (self.current_pos_y * 0.015).cos();
                    self.current_pos_x += drift;

                    if self.current_pos_y >= self.landing_y {
                        self.current_pos_y = self.landing_y;
                        self.landed = true;
                        self.with_penguin = false;
                    }
                } else {
                    self.landed = true;
                    self.hover_offset = 5.0 * (self.current_pos_y * 0.04).cos();

                    self.current_pos_y -= 0.7;

                    let drift_speed = 0.8;
                    let drift = drift_speed * (self.current_pos_y * 0.015).sin();
                    self.current_pos_x += drift;

                    if self.current_pos_y <= -self.sprite_height {
                        self.current_pos_y = -self.sprite_height;
                    }
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
