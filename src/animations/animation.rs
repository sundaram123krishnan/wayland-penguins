use std::cell::RefCell;
use std::vec;

use crate::penguin::Message;
use iced::advanced::graphics::geometry::Frame;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, column};
use iced::{
    Color, Element, Length, Point, Radians, Rectangle, Renderer, Subscription, Task, Theme,
};

use hyprland::data::*;
use hyprland::prelude::*;

use super::{
    back_forth_animation::back_forth_animation::{
        BackAndForthAnimation, BackAndForthAnimationMessage,
    },
    balloon_animation::balloon_animation::{BalloonAnimation, BalloonAnimationMessage},
};

pub struct Animation {
    draw_cache: Cache,
    back_and_forth_animation: Vec<RefCell<BackAndForthAnimation>>,
    balloon_animation: Vec<BalloonAnimation>,
    balloon_landed: Vec<RefCell<bool>>,
}

#[derive(Debug, Clone)]
pub enum AnimationMessage {
    Tick,
    BackAndForthMessage(BackAndForthAnimationMessage),
    BalloonMessage(BalloonAnimationMessage),
}

impl Animation {
    pub fn new(screen_size: (u32, u32)) -> Self {
        let back_and_forth_animation: Vec<RefCell<BackAndForthAnimation>> = (0..10)
            .map(|_| RefCell::new(BackAndForthAnimation::new(screen_size)))
            .collect();

        let balloon_animation: Vec<BalloonAnimation> = (0..10)
            .map(|_| BalloonAnimation::new(screen_size))
            .collect();
        Self {
            back_and_forth_animation,
            balloon_animation,
            balloon_landed: (0..10).map(|_| RefCell::new(false)).collect(),
            draw_cache: Default::default(),
        }
    }

    pub fn update(&mut self, message: AnimationMessage) -> Task<Message> {
        match message {
            AnimationMessage::Tick => {
                self.draw_cache.clear();
                Task::none()
            }
            AnimationMessage::BackAndForthMessage(msg) => Task::batch((0..10).map(|idx| {
                self.back_and_forth_animation[idx]
                    .borrow_mut()
                    .update(msg.clone())
            })),
            AnimationMessage::BalloonMessage(msg) => {
                Task::batch((0..10).map(|idx| self.balloon_animation[idx].update(msg.clone())))
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let back_and_forth_subscription = Subscription::batch((0..4).map(|idx| {
            self.back_and_forth_animation[idx]
                .borrow_mut()
                .subscription()
        }));
        let balloon_animation_subscription =
            Subscription::batch((0..10).map(|idx| self.balloon_animation[idx].subscription()));

        iced::Subscription::batch(vec![
            back_and_forth_subscription,
            iced::time::every(std::time::Duration::from_millis(16))
                .map(|_| Message::PlayAnimationMessage(AnimationMessage::Tick)),
            balloon_animation_subscription,
        ])
    }

    pub fn view(&self) -> Element<Message> {
        let content = column![canvas(self).height(Length::Fill).width(Length::Fill)];
        content.into()
    }

    fn draw_balloon_and_penguin(&self, frame: &mut Frame<Renderer>, idx: usize) {
        if self.balloon_animation[idx].landed {
            if *self.balloon_landed[idx].borrow() == false {
                *self.balloon_landed[idx].borrow_mut() = true;
                self.back_and_forth_animation[idx]
                    .borrow_mut()
                    .current_pos_x = self.balloon_animation[idx].current_pos_x;
            }
            let image_handle = self.back_and_forth_animation[idx]
                .borrow()
                .get_current_image_handle();
            let image = iced::advanced::image::Image {
                handle: image_handle,
                filter_method: Default::default(),
                rotation: Radians(0.0f32),
                opacity: 2.0,
                snap: false,
            };

            frame.draw_image(
                Rectangle {
                    x: self.back_and_forth_animation[idx].borrow().current_pos_x,
                    y: self.back_and_forth_animation[idx].borrow().current_pos_y,
                    width: self.back_and_forth_animation[idx].borrow().sprite_height,
                    height: self.back_and_forth_animation[idx].borrow().sprite_width,
                },
                image,
            );

            let balloon_image_handle = self.balloon_animation[idx].balloon_without_penguin.clone();
            let balloon_image = iced::advanced::image::Image {
                handle: balloon_image_handle,
                filter_method: Default::default(),
                rotation: Radians(0.0f32),
                opacity: 2.0,
                snap: false,
            };

            frame.draw_image(
                Rectangle {
                    x: self.balloon_animation[idx].current_pos_x,
                    y: self.balloon_animation[idx].current_pos_y,
                    width: self.balloon_animation[idx].sprite_height,
                    height: self.balloon_animation[idx].sprite_width,
                },
                balloon_image,
            );
        } else {
            let CursorPosition { x, y } = CursorPosition::get().unwrap();

            println!(
                "cursor_x : {}, cursor_y: {}, balloon_x: {}, balloon_y: {}",
                x,
                y,
                self.balloon_animation[idx].current_pos_x,
                self.balloon_animation[idx].current_pos_y
            );

            let balloon_image_handle = self.balloon_animation[idx].balloon_with_penguin.clone();
            let balloon_image = iced::advanced::image::Image {
                handle: balloon_image_handle,
                filter_method: Default::default(),
                rotation: Radians(0.0f32),
                opacity: 2.0,
                snap: false,
            };

            frame.draw_image(
                Rectangle {
                    x: self.balloon_animation[idx].current_pos_x,
                    y: self.balloon_animation[idx].current_pos_y,
                    width: self.balloon_animation[idx].sprite_height,
                    height: self.balloon_animation[idx].sprite_width,
                },
                balloon_image,
            );
        }
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
            for i in 0..self.back_and_forth_animation.len() {
                self.draw_balloon_and_penguin(frame, i);
            }
        });

        vec![screen]
    }
}
