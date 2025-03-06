use crate::Message::Tick;
use iced::advanced::widget::Id;
use iced::mouse::Cursor;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{button, canvas, column, image, text, Image};
use iced::window::{self, get_latest};
use iced::{
    Color, Element, Length, Point, Radians, Rectangle, Renderer, Size, Subscription, Task, Theme,
};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::{to_layer_message, Application};
use std::sync::LazyLock;

const PENGUIN: &[u8] = include_bytes!("../assets/pngwing.com.png");
static PENGUIN_HANDLE: LazyLock<image::Handle> =
    LazyLock::new(|| image::Handle::from_bytes(PENGUIN));

fn main() {
    AnimatePenguin::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((1900, 1080)),
            events_transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap()
}
#[derive(Default)]
struct AnimatePenguin {
    draw_cache: Cache,
    move_x: f32,
    move_y: f32,
    screen_size: Size,
}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    ScreenSizeReceived(Size),
}

impl Application for AnimatePenguin {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Task<Self::Message>) {
        (
            Self {
                move_y: 950.0,
                ..Default::default()
            },
            window::get_size(window::Id::unique()).map(|size| Message::ScreenSizeReceived(size)),
        )
    }
    fn style(&self, theme: &iced::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        }
    }
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // iced::time::every(std::time::Duration::from_millis(10))
        //     .map(|_| Message::Tick)

        iced::Subscription::batch(vec![iced::time::every(std::time::Duration::from_millis(
            10,
        ))
        .map(|_| Message::Tick)])
    }

    fn namespace(&self) -> String {
        String::from("Penguins Animation")
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        return match message {
            Message::Tick => {
                self.move_x += 1.0;
                self.draw_cache.clear();
                Task::none()
            }

            Message::ScreenSizeReceived(size) => {
                
                self.screen_size = size;
                println!("{} {}", size.width, size.height);
                Task::none()
            }
            _ => todo!(),
        };
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
       
        column![canvas(self).height(Length::Fill).width(Length::Fill),].into()
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

            let image = iced::advanced::image::Image {
                handle: PENGUIN_HANDLE.clone(),
                filter_method: Default::default(),
                rotation: Radians(0.0f32),
                opacity: 1.0,
                snap: false,
            };

            frame.draw_image(
                Rectangle {
                    x: self.move_x,
                    y: 950.0,
                    width: 50.0,
                    height: 50.0,
                },
                image,
            );
        });

        vec![screen]
    }
}
