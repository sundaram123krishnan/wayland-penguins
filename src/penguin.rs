use crate::animations::animation::{Animation, AnimationMessage};
use crate::widgets::modal::modal;
use iced::widget::{column, container, text};
use iced::{Color, Element, Renderer, Size, Subscription, Task, Theme};
use iced_layershell::{to_layer_message, Application};

pub struct AnimatePenguin {
    show_menu: bool,
    screen_size: (u32, u32),
    animation: Animation,
}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    ScreenSizeReceived(Size),
    PlayAnimationMessage(AnimationMessage),
    ShowMenu,
    HideMenu,
}

impl Application for AnimatePenguin {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = (u32, u32);

    fn new(flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let screen_size = flags;

        (
            Self {
                show_menu: false,
                screen_size,
                animation: Animation::new(flags),
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
        Subscription::batch(vec![self.animation.subscription()])
        // 1000ms / 16ms approx 60 fps
    }

    fn namespace(&self) -> String {
        String::from("Penguins Animation")
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        return match message {
            Message::HideMenu => {
                self.show_menu = false;
                Task::none()
            }
            Message::ShowMenu => {
                self.show_menu = true;
                Task::none()
            }
            Message::PlayAnimationMessage(msg) => self.animation.update(msg),
            _ => Task::none(),
        };
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        let x = (self.screen_size.0 as f32) / 2.5;
        let y = (self.screen_size.1 as f32) / 2.5;
        let content = self.animation.view();

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
