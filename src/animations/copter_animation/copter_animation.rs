use super::copter_animation_assets::get_penguin_copter_image;
use crate::penguin::Message;
use iced::widget::image;
use iced::Task;
use rand::Rng;
use std::f32::consts::TAU;

pub struct CopterAnimation {
    pub current_pos_x: f32,
    pub current_pos_y: f32,
    pub copter_asset: image::Handle,
    pub sprite_height: f32,
    pub sprite_width: f32,
    landed: bool,
    landing_y: f32,
    velocity_y: f32,
    base_x: f32,
    time: f32,
    hover_offset_y: f32,
    rotation_angle: f32,
    wind_phase: f32,
}

#[derive(Debug, Clone)]
pub enum CopterAnimationMessage {
    Tick,
}

impl CopterAnimation {
    pub fn new(screen_size: (u32, u32)) -> Self {
        let copter_asset = get_penguin_copter_image();
        let sprite_height = 60.0;
        let sprite_width = 60.0;
        let screen_x = screen_size.0;
        let left_margin = sprite_width * 1.75;
        let right_margin = screen_x as f32 - (sprite_width * 1.75);
        let mut rnd = rand::rng();
        let random_x = rnd.random_range(left_margin..right_margin);
        let landing_y = screen_size.1 as f32 - 70.0;

        Self {
            current_pos_x: random_x,
            current_pos_y: 10.0,
            copter_asset,
            sprite_width,
            sprite_height,
            landed: false,
            landing_y,
            velocity_y: 0.0,
            base_x: random_x,
            time: 0.0,
            hover_offset_y: 0.0,
            rotation_angle: 0.0,
            wind_phase: rnd.random_range(0.0..TAU),
        }
    }

    pub fn update(&mut self, message: CopterAnimationMessage) -> Task<Message> {
        match message {
            CopterAnimationMessage::Tick => {
                self.time += 0.1;

                if !self.landed {
                    self.velocity_y += 0.08;
                    self.velocity_y = self.velocity_y.min(3.0);
                    self.current_pos_y += self.velocity_y;

                    let sway_amplitude = 10.0 + (self.time * 0.3).sin() * 5.0;
                    let sway_frequency = 0.05;
                    self.current_pos_x = self.base_x;

                    let bob_amplitude = 2.0;
                    let bob_frequency = 0.15;
                    let bob_offset = (self.time * bob_frequency).sin() * bob_amplitude;

                    let horizontal_velocity = (self.time * sway_frequency + self.wind_phase).cos()
                        * sway_amplitude
                        * sway_frequency;
                    self.rotation_angle = horizontal_velocity * 0.1;

                    self.current_pos_y += bob_offset;

                    let distance_to_ground = self.landing_y - self.current_pos_y;
                    if distance_to_ground < 30.0 {
                        self.velocity_y *= 0.92;
                        self.rotation_angle += (distance_to_ground / 30.0) * 0.1;
                    }

                    if self.current_pos_y >= self.landing_y {
                        self.landed = true;
                        self.current_pos_y = self.landing_y;
                        self.velocity_y = 0.0;
                    }
                } else {
                    let hover_amplitude = 3.0;
                    let hover_frequency = 0.08;
                    self.hover_offset_y = (self.time * hover_frequency).sin() * hover_amplitude;

                    self.current_pos_x = self.base_x;

                    if self.time > 10.0 {
                        self.velocity_y -= 0.15;
                        self.velocity_y = self.velocity_y.max(-6.0);
                        self.current_pos_y += self.velocity_y;

                        let ascent_sway_amplitude = 20.0;
                        let ascent_sway_frequency = 0.07;
                        self.current_pos_x = self.base_x;

                        let ascent_horizontal_velocity =
                            (self.time * ascent_sway_frequency + self.wind_phase + 3.0).cos()
                                * ascent_sway_amplitude
                                * ascent_sway_frequency;
                        self.rotation_angle = ascent_horizontal_velocity * 0.12;

                        if self.current_pos_y <= -self.sprite_height * 2.0 {
                            self.current_pos_y = -self.sprite_height * 2.0;
                        }
                    } else {
                        self.current_pos_y = self.landing_y + self.hover_offset_y;
                    }
                }

                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(16)).map(|_| {
            Message::PlayAnimationMessage(
                crate::animations::animation::AnimationMessage::CopterMessage(
                    CopterAnimationMessage::Tick,
                ),
            )
        })
    }
}
