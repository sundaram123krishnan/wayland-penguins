use crate::assets::get_penguin_image;
use crate::widgets::modal::modal;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, column, container, image, text};
use iced::{Color, Element, Length, Point, Radians, Rectangle, Renderer, Size, Task, Theme};
use iced_layershell::{to_layer_message, Application};

#[derive(Default)]
pub struct AnimatePenguin {
    draw_cache: Cache,
    move_x: f32,
    move_y: f32,
    screen_size: (u32, u32),
    show_menu: bool,
    frame_counter: usize,
    sprite_height: f32,
    sprite_width: f32,
    right_walking_image_handle: Vec<image::Handle>,
    right_to_front_image_handle: Vec<image::Handle>,
    left_walking_image_handle: Vec<image::Handle>,
    direction: AnimationState,
    turn_point: f32,
}

#[derive(Default, PartialEq)]
pub enum AnimationState {
    RightToFront,
    FrontToLeft,
    LeftAnimation,
    FrontTalking,
    FrontToRight,
    #[default]
    RightAnimation,
}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    ScreenSizeReceived(Size),
    ShowMenu,
    HideMenu,
    RestartAnimation,
}

impl AnimatePenguin {
    pub fn x_pos(&mut self, animation_state: AnimationState) {
        match animation_state {
            AnimationState::RightAnimation => {
                self.move_x += 0.6;
            }
            AnimationState::LeftAnimation => {
                self.move_x -= 0.6;
                self.turn_point -= 0.6;
            }
            _ => {}
        }
    }
}

impl Application for AnimatePenguin {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = (u32, u32);

    fn new(flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let bottom = flags.1 as f32 - 60.0f32;

        let right_walking_image_handle = get_penguin_image(AnimationState::RightAnimation);
        let right_to_front_image_handle = get_penguin_image(AnimationState::RightToFront);
        let left_walking_image_handle = get_penguin_image(AnimationState::LeftAnimation);

        (
            Self {
                screen_size: flags,
                show_menu: false,
                move_y: bottom,
                sprite_height: 60.0,
                sprite_width: 60.0,
                frame_counter: 0,
                right_walking_image_handle,
                right_to_front_image_handle,
                left_walking_image_handle,
                turn_point: 400.0, // TODO: should be randomized
                direction: AnimationState::RightAnimation,
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn style(&self, _theme: &iced::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_millis(16)).map(|_| Message::Tick)
        // 1000ms / 16ms approx 60 fps
    }

    fn namespace(&self) -> String {
        String::from("Penguins Animation")
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        return match message {
            Message::Tick => {
                if !self.show_menu { 
                    if self.move_x >= self.turn_point && self.turn_point > 1.0 {
                        self.direction = AnimationState::LeftAnimation;
                        self.x_pos(AnimationState::LeftAnimation);
                    } else {
                        self.turn_point = 400.0;
                        self.direction = AnimationState::RightAnimation;
                        self.x_pos(AnimationState::RightAnimation);
                    }
                    
                    self.frame_counter += 1;

                    if self.frame_counter >= self.left_walking_image_handle.len() {
                        self.frame_counter = 0;
                    }

                    self.draw_cache.clear();
                }
                Task::none()
            }
            Message::HideMenu => {
                self.show_menu = false;
                Task::none()
            }
            Message::ShowMenu => {
                self.show_menu = true;
                Task::none()
            }
            _ => Task::none(),
        };
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        let x = (self.screen_size.0 as f32) / 2.5;
        let y = (self.screen_size.1 as f32) / 2.5;
        let content = column![canvas(self).height(Length::Fill).width(Length::Fill),];

        if self.show_menu {
            // TODO
            let menu = container(column![text("Penguin Walking Animation").size(24),].spacing(20))
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(
                        1.0, 1.0, 1.0, 0.9,
                    ))),
                    text_color: Some(Color::BLACK),
                    ..container::Style::default()
                })
                .width(x as f32)
                .height(y as f32)
                .padding(20);

            modal(content, menu, Message::HideMenu).into()
        } else {
            content.into()
        }
    }
}

impl<Message> canvas::Program<Message> for AnimatePenguin {
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

            let mut image_handle = self.right_walking_image_handle[self.frame_counter].clone();

            if self.direction == AnimationState::LeftAnimation {
                image_handle = self.left_walking_image_handle[self.frame_counter].clone();
            }

            let image = iced::advanced::image::Image {
                handle: image_handle,
                filter_method: Default::default(),
                rotation: Radians(0.0f32),
                opacity: 1.0,
                snap: false,
            };

            frame.draw_image(
                Rectangle {
                    x: self.move_x,
                    y: self.move_y,
                    width: self.sprite_height,
                    height: self.sprite_width,
                },
                image,
            );
        });

        vec![screen]
    }
}
