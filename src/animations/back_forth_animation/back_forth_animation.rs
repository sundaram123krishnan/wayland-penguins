use crate::animations::animation::AnimationMessage;
use crate::animations::back_forth_animation::back_forth_assets::get_penguin_image;
use crate::penguin::Message;
use iced::widget::image;
use iced::Task;
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
    start_point: f32,
    screen_size: (u32, u32),
    pub current_pos_x: f32, // current x-coord position of the penguin
    pub current_pos_y: f32, // current y-coord position of the penguin
    frame_counter: i32, // to play frames -> stays between 0 - 40 (as we have 40 frames for each animation)
    previous_start_point: f32,
    pub sprite_height: f32,
    pub sprite_width: f32,
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
    rng.random_range(300..screen_size_x - 50) as i32
}

fn randomize_start_point(turn_point: i32) -> f32 {
    let mut rng = rand::rng();
    rng.random_range(5..turn_point - 300) as f32
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
        }
    }

    pub fn update(&mut self, message: BackAndForthAnimationMessage) -> Task<Message> {
        match message {
            BackAndForthAnimationMessage::Tick => {
                self.update_animation_state();
                self.update_position();
                self.update_frame_counter();
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(16)).map(|_| {
            Message::PlayAnimationMessage(AnimationMessage::BackAndForthMessage(
                BackAndForthAnimationMessage::Tick,
            ))
        })
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

    // whenever animation is going from front-right, reset
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
            BackAndForthAnimationState::FrontToLeft => {
                self.current_pos_x -= self.animation_speed * 0.5; // move current position when turning
                self.counter += 1;
            }
            BackAndForthAnimationState::FrontToRight => {
                self.current_pos_x += self.animation_speed * 0.5; // still move the current position when turning
                self.counter += 1;
            }
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
            self.counter = 2 * self.turn_point - 60;
        }
        self.current_pos_x -= self.animation_speed;
        self.counter += 1;
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

    pub fn get_current_image_handle(&self) -> image::Handle {
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
