use crate::animations::back_forth_animation::back_forth_assets::get_penguin_image;
use crate::penguin::Message;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, column, image};
use iced::{Color, Element, Length, Point, Radians, Rectangle, Renderer, Task, Theme};
use rand::Rng;

// The animation states of the sprite
#[derive(Default, PartialEq)]
pub enum BackAndForthAnimationState {
    RightToFront,
    FrontToLeft,
    LeftAnimation,
    FrontTalking, // TODO
    FrontToRight,
    LeftToFront,
    #[default]
    RightAnimation,
}

#[derive(Default)]
pub struct BackAndForthAnimation {
    draw_cache: Cache,
    start_point: f32,
    screen_size: (u32, u32),
    current_pos_x: f32, // current x-coord position of the penguin
    current_pos_y: f32, // current y-coord position of the penguin
    frame_counter: i32, // to play frames -> stays between 0 - 40 (as we have 40 frames for each animation)
    previous_start_point: f32,
    sprite_height: f32,
    sprite_width: f32,
    next_start_point: f32,
    animation_speed: f32,
    right_walking_image_handle: Vec<image::Handle>,
    right_to_front_image_handle: Vec<image::Handle>,
    left_walking_image_handle: Vec<image::Handle>,
    front_to_left_image_handle: Vec<image::Handle>,
    left_to_front_image_handle: Vec<image::Handle>,
    front_to_right_image_handle: Vec<image::Handle>,
    direction: BackAndForthAnimationState, // holds current animation state value
    counter: i32,                          // increments on each tick, to change animation states
    turn_point: i32,                       // switch directions between left-right
    should_go_left: bool,
}

#[derive(Debug, Clone)]
pub enum BackAndForthAnimationMessage {
    Tick,
}

fn randomize_turn_point(screen_size_x: u32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(100..600) as i32
}

fn randomize_start_point(turn_point: i32) -> f32 {
    let mut rng = rand::rng();
    if turn_point <= 100 {
        0.0
    } else {
        rng.random_range(0..turn_point - 100) as f32
    }
}

impl BackAndForthAnimation {
    pub fn new(screen_size: (u32, u32)) -> Self {
        let bottom = screen_size.1 as f32 - 50.0f32; // place the penguin in bottom
        let turn_point = randomize_turn_point(screen_size.0);
        let start_point = randomize_start_point(turn_point);

        Self {
            start_point,
            current_pos_y: bottom,
            sprite_height: 50.0,
            sprite_width: 50.0,
            frame_counter: 0,
            screen_size,
            previous_start_point: start_point,
            next_start_point: start_point,
            animation_speed: 1.0,
            right_walking_image_handle: get_penguin_image(
                BackAndForthAnimationState::RightAnimation,
            ),
            right_to_front_image_handle: get_penguin_image(
                BackAndForthAnimationState::RightToFront,
            ),
            left_walking_image_handle: get_penguin_image(BackAndForthAnimationState::LeftAnimation),
            front_to_left_image_handle: get_penguin_image(BackAndForthAnimationState::FrontToLeft),
            left_to_front_image_handle: get_penguin_image(BackAndForthAnimationState::LeftToFront),
            front_to_right_image_handle: get_penguin_image(
                BackAndForthAnimationState::FrontToRight,
            ),
            should_go_left: false,
            direction: BackAndForthAnimationState::RightAnimation,
            counter: 0,
            turn_point,
            current_pos_x: 0.0,
            draw_cache: Default::default(),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(16))
            .map(|_| Message::BackAndForthMessage(BackAndForthAnimationMessage::Tick))
        // 1000ms / 16ms approx 60 fps
    }

    pub fn update(&mut self, message: BackAndForthAnimationMessage) -> Task<Message> {
        match message {
            BackAndForthAnimationMessage::Tick => {
                self.update_animation_state();
                self.update_position();
                self.update_frame_counter();
                self.draw_cache.clear();
                Task::none()
            }
        }
    }

    fn update_animation_state(&mut self) {
        if self.is_transitioning_to_front_from_left() {
            self.direction = BackAndForthAnimationState::LeftToFront;
        } else if self.is_transitioning_to_right_from_front() {
            self.direction = BackAndForthAnimationState::FrontToRight;
        } else if self.is_transitioning_to_front_from_right() {
            self.direction = BackAndForthAnimationState::RightToFront;
        } else if self.is_transitioning_to_left_from_front() {
            self.direction = BackAndForthAnimationState::FrontToLeft;
        } else if self.is_in_left_animation_phase() {
            self.direction = BackAndForthAnimationState::LeftAnimation;
        } else if self.should_reset_animation() {
            self.reset_animation();
        }
    }

    fn is_transitioning_to_front_from_left(&self) -> bool {
        self.counter >= (2 * self.turn_point - 60) && self.counter <= (2 * self.turn_point - 36)
    }

    fn is_transitioning_to_right_from_front(&self) -> bool {
        self.counter > (2 * self.turn_point - 36) && self.counter <= (2 * self.turn_point - 12)
    }

    fn is_transitioning_to_front_from_right(&self) -> bool {
        self.counter >= self.turn_point - 48 && self.counter < self.turn_point - 24
    }

    fn is_transitioning_to_left_from_front(&self) -> bool {
        self.counter >= self.turn_point - 24 && self.counter < self.turn_point
    }

    fn is_in_left_animation_phase(&self) -> bool {
        self.counter >= self.turn_point && self.counter < (2 * self.turn_point - 60)
    }

    fn should_reset_animation(&self) -> bool {
        self.direction == BackAndForthAnimationState::FrontToRight
            && self.counter > (2 * self.turn_point - 12)
    }

    fn reset_animation(&mut self) {
        self.counter = 0;
        self.previous_start_point = self.start_point;
        self.next_start_point = randomize_start_point(self.turn_point);
        self.start_point = self.next_start_point;
        self.turn_point = randomize_turn_point(self.screen_size.0);
        self.direction = BackAndForthAnimationState::RightAnimation;
        self.should_go_left = self.previous_start_point > self.next_start_point;
        self.frame_counter = 0;
    }

    fn update_position(&mut self) {
        match self.direction {
            BackAndForthAnimationState::RightAnimation => self.update_right_animation_position(),
            BackAndForthAnimationState::LeftAnimation => self.update_left_animation_position(),
            _ => self.counter += 1,
        }
    }

    fn update_right_animation_position(&mut self) {
        if self.should_go_left && self.counter >= 30 {
            self.counter = self.turn_point - 48;
            self.should_go_left = false;
        } else {
            self.current_pos_x += self.animation_speed;
            self.counter += 1;
        }
    }

    fn update_left_animation_position(&mut self) {
        if self.current_pos_x <= self.start_point {
            println!("Old start point is : {:?}", self.start_point);
            self.turn_point = randomize_turn_point(self.screen_size.0);
            self.start_point = randomize_start_point(self.turn_point);
            println!(
                "New start poing is : {:?}, current position is: {:?}",
                self.start_point, self.current_pos_x
            );
            self.direction = BackAndForthAnimationState::RightAnimation;
            self.counter = 0;
            self.frame_counter = 0;
        } else {
            self.current_pos_x -= self.animation_speed;
            self.counter += 1;
        }
    }

    fn update_frame_counter(&mut self) {
        let total_frames = 40;

        // the frame counter should always be between 0 - 40 (no overflow)
        self.frame_counter = match self.direction {
            BackAndForthAnimationState::RightToFront => {
                let fc = ((self.counter - (self.turn_point - 48)) * total_frames) / 24;
                fc.min(39)
            }
            BackAndForthAnimationState::FrontToLeft => {
                let fc = ((self.counter - (self.turn_point - 24)) * total_frames) / 24;
                fc.min(39)
            }
            BackAndForthAnimationState::LeftToFront => {
                let fc = ((self.counter - (2 * self.turn_point - 60)) * total_frames) / 24;
                fc.min(39)
            }
            BackAndForthAnimationState::FrontToRight => {
                let fc = ((self.counter - (2 * self.turn_point - 36)) * total_frames) / 24;
                fc.min(39)
            }
            _ => self.counter % total_frames,
        };
    }

    pub fn view(&self) -> Element<Message> {
        let content = column![canvas(self).height(Length::Fill).width(Length::Fill)];
        content.into()
    }

    fn get_current_image_handle(&self) -> image::Handle {
        match self.direction {
            BackAndForthAnimationState::LeftAnimation => {
                self.left_walking_image_handle[self.frame_counter as usize].clone()
            }
            BackAndForthAnimationState::RightToFront => {
                self.right_to_front_image_handle[self.frame_counter as usize].clone()
            }
            BackAndForthAnimationState::FrontToLeft => {
                self.front_to_left_image_handle[self.frame_counter as usize].clone()
            }
            BackAndForthAnimationState::LeftToFront => {
                self.left_to_front_image_handle[self.frame_counter as usize].clone()
            }
            BackAndForthAnimationState::FrontToRight => {
                self.front_to_right_image_handle[self.frame_counter as usize].clone()
            }
            _ => self.right_walking_image_handle[self.frame_counter as usize].clone(),
        }
    }
}

impl<Message> canvas::Program<Message> for BackAndForthAnimation {
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

            let image_handle = self.get_current_image_handle();

            let image = iced::advanced::image::Image {
                handle: image_handle,
                filter_method: Default::default(),
                rotation: Radians(0.0f32),
                opacity: 1.0,
                snap: false,
            };

            frame.draw_image(
                Rectangle {
                    x: self.current_pos_x,
                    y: self.current_pos_y,
                    width: self.sprite_height,
                    height: self.sprite_width,
                },
                image,
            );
        });

        vec![screen]
    }
}
