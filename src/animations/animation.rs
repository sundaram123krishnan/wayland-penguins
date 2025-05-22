use std::cell::RefCell;
use std::fs::read;
use std::vec;

use crate::penguin::Message;
use hyprland::data::{Client, Clients};
use iced::advanced::graphics::geometry::Frame;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, column, image};
use iced::{
    Color, Element, Length, Point, Radians, Rectangle, Renderer, Subscription, Task, Theme,
};

use super::{
    back_forth_animation::back_forth_animation::{
        BackAndForthAnimation, BackAndForthAnimationMessage,
    },
    balloon_animation::balloon_animation::{BalloonAnimation, BalloonAnimationMessage},
};

use hyprland::data::*;
use hyprland::prelude::*;

const TOTAL_PENGUINS: usize = 1;

pub struct Animation {
    draw_cache: Cache,
    back_and_forth_animation: Vec<RefCell<BackAndForthAnimation>>,
    balloon_animation: Vec<BalloonAnimation>,
    balloon_landed: Vec<RefCell<bool>>,
    half_bottom_window_clients: Vec<Client>,
    penguin_copter: image::Handle,
    screen_size: (u32, u32),
}

#[derive(Debug, Clone)]
pub enum AnimationMessage {
    Tick,
    BackAndForthMessage(BackAndForthAnimationMessage),
    BalloonMessage(BalloonAnimationMessage),
}

fn is_small_window(screen_size: (u32, u32), client: &Client) -> bool {
    let screen_width = screen_size.0 as f32;
    let screen_height = screen_size.1 as f32;
    let screen_area = (screen_width * screen_height) as f64;

    let client_area = client.size.0 as f64 * client.size.1 as f64;

    let area_ratio = client_area / screen_area;

    area_ratio <= 0.33 && client.at.1 >= (screen_height / 4.5) as i16
}

fn get_half_bottom_window_clients(screen_size: (u32, u32)) -> Vec<Client> {
    let window_clients = Clients::get().unwrap().to_vec();

    let half_bottom_window_clients: Vec<Client> = window_clients
        .clone()
        .into_iter()
        .filter(|client| is_small_window(screen_size, client))
        .collect();

    half_bottom_window_clients
}

fn get_penguin_copter_image() -> image::Handle {
    let root = std::env::current_dir().unwrap();

    let assets_dir = root.join("assets").join("PenguinCopter");
    let asset_path = assets_dir.join("pixelated_penguin_copter.png");
    let image_bytes = read(&asset_path).unwrap();
    image::Handle::from_bytes(image_bytes)
}

impl Animation {
    pub fn new(screen_size: (u32, u32)) -> Self {
        let window_clients = Clients::get().unwrap().to_vec();
        let y_pos = window_clients[0].size.1;

        let back_and_forth_animation: Vec<RefCell<BackAndForthAnimation>> = (0..TOTAL_PENGUINS)
            .map(|_| RefCell::new(BackAndForthAnimation::new(screen_size, y_pos)))
            .collect();

        let balloon_animation: Vec<BalloonAnimation> = (0..TOTAL_PENGUINS)
            .map(|_| BalloonAnimation::new(screen_size))
            .collect();

        Self {
            back_and_forth_animation,
            balloon_animation,
            balloon_landed: (0..TOTAL_PENGUINS).map(|_| RefCell::new(false)).collect(),
            draw_cache: Default::default(),
            half_bottom_window_clients: get_half_bottom_window_clients(screen_size),
            screen_size,
            penguin_copter: get_penguin_copter_image(),
        }
    }

    pub fn update(&mut self, message: AnimationMessage) -> Task<Message> {
        match message {
            AnimationMessage::Tick => {
                self.draw_cache.clear();
                self.half_bottom_window_clients = get_half_bottom_window_clients(self.screen_size);
                Task::none()
            }
            AnimationMessage::BackAndForthMessage(msg) => {
                Task::batch((0..TOTAL_PENGUINS).map(|idx| {
                    self.back_and_forth_animation[idx]
                        .borrow_mut()
                        .update(msg.clone())
                }))
            }
            AnimationMessage::BalloonMessage(msg) => Task::batch(
                (0..TOTAL_PENGUINS).map(|idx| self.balloon_animation[idx].update(msg.clone())),
            ),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let back_and_forth_subscription = Subscription::batch((0..TOTAL_PENGUINS).map(|idx| {
            self.back_and_forth_animation[idx]
                .borrow_mut()
                .subscription()
        }));
        let balloon_animation_subscription = Subscription::batch(
            (0..TOTAL_PENGUINS).map(|idx| self.balloon_animation[idx].subscription()),
        );

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
        println!("{:?}", self.half_bottom_window_clients.len());
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

            let cursor_pos_x = self.balloon_animation[idx].current_pos_x
                + self.balloon_animation[idx].sprite_width / 2.0;
            let cursor_pos_y = self.balloon_animation[idx].current_pos_y
                + self.balloon_animation[idx].sprite_height / 2.0;

            if (x as f32 - cursor_pos_x).abs() <= 30.0 && (y as f32 - cursor_pos_y).abs() <= 30.0 {
                let balloon_image_handle = self.balloon_animation[idx]
                    .balloon_with_hyprland_logo
                    .clone();
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
