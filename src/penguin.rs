use crate::animations::animation::{Animation, AnimationMessage};
use crate::widgets::modal::modal;
use iced::widget::{column, container, text};
use iced::{Color, Element, Size, Subscription, Task};
use iced_layershell::to_layer_message;
use std::sync::OnceLock;
pub struct AnimatePenguin {
    show_menu: bool,
    screen_size: Option<Size>,
    animation: Option<Animation>,
    mainwindow: OnceLock<iced::window::Id>,
}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    ScreenSizeReceived(Size),
    PlayAnimationMessage(AnimationMessage),
    ShowMenu,
    HideMenu,
    LatestWindow(Option<iced::window::Id>),
    SizeUpdate(iced::Size),
}

impl AnimatePenguin {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                mainwindow: OnceLock::new(),
                show_menu: false,
                screen_size: None,
                animation: None,
            },
            iced::window::get_latest().map(Message::LatestWindow),
        )
    }

    pub fn style(&self, _theme: &iced::Theme) -> iced::theme::Style {
        use iced::theme::Style;
        Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        match &self.animation {
            Some(animation) => animation.subscription(),
            None => Subscription::none(),
        }
        // 1000ms / 16ms approx 60 fps
    }

    pub fn namespace() -> String {
        String::from("Penguins Animation")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        return match message {
            Message::HideMenu => {
                self.show_menu = false;
                Task::none()
            }
            Message::ShowMenu => {
                self.show_menu = true;
                Task::none()
            }
            Message::PlayAnimationMessage(msg) => match &mut self.animation {
                Some(animation) => animation.update(msg),
                None => Task::none(),
            },
            Message::LatestWindow(id) => {
                let id = id.expect("must can get one");
                self.mainwindow.set(id).expect("We just set once");
                iced::window::get_size(id).map(Message::SizeUpdate)
            }
            Message::SizeUpdate(size) => {
                self.animation = Some(Animation::new((size.width as u32, size.height as u32)));

                self.screen_size = Some(size);
                Task::none()
            }
            _ => Task::none(),
        };
    }

    pub fn view(&self) -> Element<Message> {
        let Some(screen_size) = self.screen_size else {
            return text("").into();
        };
        let Some(animation) = &self.animation else {
            return text("").into();
        };
        let x = screen_size.width / 2.5;
        let y = screen_size.height / 2.5;

        let content = animation.view();

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
