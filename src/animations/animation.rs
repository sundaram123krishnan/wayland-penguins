use std::vec;

use crate::penguin::Message;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, column};
use iced::{Color, Element, Length, Point, Radians, Rectangle, Renderer, Task, Theme};

use super::{
    back_forth_animation::back_forth_animation::{
        BackAndForthAnimation, BackAndForthAnimationMessage,
    },
    balloon_animation::balloon_animation::{BalloonAnimation, BalloonAnimationMessage},
};

pub struct Animation {
    draw_cache: Cache,
    back_and_forth_animation: BackAndForthAnimation,
    balloon_animation: BalloonAnimation,
}

#[derive(Debug, Clone)]
pub enum AnimationMessage {
    Tick,
    BackAndForthMessage(BackAndForthAnimationMessage),
    BalloonMessage(BalloonAnimationMessage),
}

impl Animation {
    pub fn new(screen_size: (u32, u32)) -> Self {
        Self {
            back_and_forth_animation: BackAndForthAnimation::new(screen_size),
            balloon_animation: BalloonAnimation::new(screen_size),
            draw_cache: Default::default(),
        }
    }

    pub fn update(&mut self, message: AnimationMessage) -> Task<Message> {
        match message {
            AnimationMessage::Tick => {
                self.draw_cache.clear();
                Task::none()
            }
            AnimationMessage::BackAndForthMessage(msg) => self.back_and_forth_animation.update(msg),
            AnimationMessage::BalloonMessage(msg) => self.balloon_animation.update(msg),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(vec![
            self.back_and_forth_animation.subscription(),
            iced::time::every(std::time::Duration::from_millis(16))
                .map(|_| Message::PlayAnimationMessage(AnimationMessage::Tick)),
            self.balloon_animation.subscription(),
        ])
    }

    pub fn view(&self) -> Element<Message> {
        let content = column![canvas(self).height(Length::Fill).width(Length::Fill)];
        content.into()
    }
}

impl<Message> canvas::Program<Message> for Animation {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let screen = self.draw_cache.draw(renderer, bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, bounds.size());
            frame.fill(&background, Color::TRANSPARENT);

            if self.balloon_animation.landed {
                let image_handle = self.back_and_forth_animation.get_current_image_handle();
                let image = iced::advanced::image::Image {
                    handle: image_handle,
                    filter_method: Default::default(),
                    rotation: Radians(0.0f32),
                    opacity: 1.0,
                    snap: false,
                };
                frame.draw_image(
                    Rectangle {
                        x: self.back_and_forth_animation.current_pos_x,
                        y: self.back_and_forth_animation.current_pos_y,
                        width: self.back_and_forth_animation.sprite_height,
                        height: self.back_and_forth_animation.sprite_width,
                    },
                    image,
                );
            } else {
                let balloon_image_handle = self.balloon_animation.balloon_with_penguin.clone();

                let balloon_image = iced::advanced::image::Image {
                    handle: balloon_image_handle,
                    filter_method: Default::default(),
                    rotation: Radians(0.0f32),
                    opacity: 1.0,
                    snap: false,
                };
                frame.draw_image(
                    Rectangle {
                        x: self.balloon_animation.current_pos_x,
                        y: self.balloon_animation.current_pos_y,
                        width: self.balloon_animation.sprite_height,
                        height: self.balloon_animation.sprite_width,
                    },
                    balloon_image,
                );
            }
        });

        vec![screen]
    }
}
