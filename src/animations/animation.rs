use std::cell::RefCell;
use std::vec;

use crate::penguin::Message;
use iced::advanced::graphics::geometry::Frame;
use iced::border::Radius;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, column};
use iced::{
    Color, Element, Length, Point, Radians, Rectangle, Renderer, Subscription, Task, Theme,
};

use super::copter_animation::copter_animation::{CopterAnimation, CopterAnimationMessage};
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
    copter_animation: Vec<CopterAnimation>,
    balloon_landed: Vec<RefCell<bool>>,
    screen_size: (u32, u32),
    animations_to_be_spawned: i32,
    bottom_y_pos: i16,
}

#[derive(Debug, Clone)]
pub enum AnimationMessage {
    Tick,
    BackAndForthMessage(BackAndForthAnimationMessage),
    BalloonMessage(BalloonAnimationMessage),
    CopterMessage(CopterAnimationMessage),
}

impl Animation {
    pub fn new(screen_size: (u32, u32)) -> Self {
        let y_pos = (screen_size.1 as i16) - 60;

        let back_and_forth_animation: Vec<RefCell<BackAndForthAnimation>> = (0..1)
            .map(|_| RefCell::new(BackAndForthAnimation::new(screen_size, y_pos)))
            .collect();

        let balloon_animation: Vec<BalloonAnimation> =
            (0..1).map(|_| BalloonAnimation::new(screen_size)).collect();

        let copter_animation: Vec<CopterAnimation> =
            (0..1).map(|_| CopterAnimation::new(screen_size)).collect();

        Self {
            back_and_forth_animation,
            balloon_animation,
            balloon_landed: (0..7).map(|_| RefCell::new(false)).collect(),
            draw_cache: Default::default(),
            screen_size,
            animations_to_be_spawned: 0,
            copter_animation,
            bottom_y_pos: y_pos,
        }
    }

    pub fn update(&mut self, message: AnimationMessage) -> Task<Message> {
        match message {
            AnimationMessage::Tick => {
                self.draw_cache.clear();
                // This is to add a delay
                // Can't think of anything better
                // spawn more copter animation penguins
                if self.animations_to_be_spawned % 200 == 0
                    && self.animations_to_be_spawned <= 1000
                    && self.animations_to_be_spawned >= 200
                {
                    self.balloon_animation
                        .push(BalloonAnimation::new(self.screen_size));
                    self.back_and_forth_animation
                        .push(RefCell::new(BackAndForthAnimation::new(
                            self.screen_size,
                            (self.bottom_y_pos) as i16,
                        )));
                    self.copter_animation
                        .push(CopterAnimation::new(self.screen_size));
                } else if self.animations_to_be_spawned > 1000 {
                    return Task::none();
                }
                self.animations_to_be_spawned += 1;
                Task::none()
            }
            AnimationMessage::BackAndForthMessage(msg) => {
                Task::batch((0..self.back_and_forth_animation.len()).map(|idx| {
                    self.back_and_forth_animation[idx]
                        .borrow_mut()
                        .update(msg.clone())
                }))
            }
            AnimationMessage::BalloonMessage(msg) => Task::batch(
                (0..self.balloon_animation.len())
                    .map(|idx| self.balloon_animation[idx].update(msg.clone())),
            ),
            AnimationMessage::CopterMessage(msg) => Task::batch(
                (0..self.copter_animation.len())
                    .map(|idx| self.copter_animation[idx].update(msg.clone())),
            ),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let back_and_forth_subscription =
            Subscription::batch((0..self.back_and_forth_animation.len()).map(|idx| {
                self.back_and_forth_animation[idx]
                    .borrow_mut()
                    .subscription()
            }));
        let balloon_animation_subscription = Subscription::batch(
            (0..self.balloon_animation.len()).map(|idx| self.balloon_animation[idx].subscription()),
        );
        let copter_animation_subscription = Subscription::batch(
            (0..self.copter_animation.len()).map(|idx| self.copter_animation[idx].subscription()),
        );

        iced::Subscription::batch(vec![
            back_and_forth_subscription,
            iced::time::every(std::time::Duration::from_millis(16))
                .map(|_| Message::PlayAnimationMessage(AnimationMessage::Tick)),
            balloon_animation_subscription,
            copter_animation_subscription,
        ])
    }

    pub fn view(&'_ self) -> Element<'_, Message> {
        let content = column![canvas(self).height(Length::Fill).width(Length::Fill)];
        content.into()
    }

    fn draw_balloon_and_penguin(&self, frame: &mut Frame<Renderer>, idx: usize) {
        if self.balloon_animation[idx].landed {
            let back_forth_y_pos = self.back_and_forth_animation[idx].borrow().current_pos_y;

            if *self.balloon_landed[idx].borrow() == false {
                *self.balloon_landed[idx].borrow_mut() = true;

                let mut back_and_forth_pos = self.back_and_forth_animation[idx].borrow_mut();

                let balloon_x_pos = self.balloon_animation[idx].current_pos_x;

                back_and_forth_pos.current_pos_x = balloon_x_pos;
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
                border_radius: Radius::default(),
            };

            frame.draw_image(
                Rectangle {
                    x: self.back_and_forth_animation[idx].borrow().current_pos_x,
                    y: back_forth_y_pos,
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
                border_radius: Radius::default(),
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
            let copter_image_handle = self.copter_animation[idx].copter_asset.clone();
            let copter_image = iced::advanced::image::Image {
                handle: copter_image_handle,
                filter_method: Default::default(),
                opacity: 1.0,
                snap: false,
                rotation: Radians(0.0f32),
                border_radius: Radius::default(),
            };
            frame.draw_image(
                Rectangle {
                    x: self.copter_animation[idx].current_pos_x,
                    y: self.copter_animation[idx].current_pos_y,
                    width: self.copter_animation[idx].sprite_height,
                    height: self.copter_animation[idx].sprite_width,
                },
                copter_image,
            );
            let balloon_image_handle = self.balloon_animation[idx].balloon_with_penguin.clone();
            let balloon_image = iced::advanced::image::Image {
                handle: balloon_image_handle,
                filter_method: Default::default(),
                rotation: Radians(0.0f32),
                opacity: 2.0,
                snap: false,

                border_radius: Radius::default(),
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
            for i in 0..self.balloon_animation.len() {
                self.draw_balloon_and_penguin(frame, i);
            }
        });

        vec![screen]
    }
}
