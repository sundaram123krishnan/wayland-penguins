use iced::{Color, Element, Length, Point, Radians, Rectangle, Renderer, Size, Task, Theme};
use iced_layershell::{to_layer_message, Application};
use iced::widget::{text, column, button, image, canvas, Image};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use std::sync::LazyLock;
use iced::mouse::Cursor;
use iced::widget::canvas::{Cache, Geometry, Path};

const PENGUIN: &[u8] = include_bytes!("../assets/pngwing.com.png");
static PENGUIN_HANDLE: LazyLock<image::Handle> =
    LazyLock::new(|| image::Handle::from_bytes(PENGUIN));

fn main() {
    AnimatePenguin::run(
        Settings {
            layer_settings: LayerShellSettings {
                size: Some((1900, 1080)),
                events_transparent: true,
                ..Default::default()
            },
                ..Default::default()
        }
    ).unwrap()
}
#[derive(Default)]
struct AnimatePenguin {
    draw_cache: Cache,
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {}

impl Application for AnimatePenguin {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Task<Self::Message>) {
        (
            Self {
                ..Default::default()
            }, Task::none()
        )
    }
    fn style(&self, theme: &iced::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        }
    }

    fn namespace(&self) -> String {
        String::from("Penguins Animation")
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        // column![
        //     text("hello"),
        //    image(PENGUIN_HANDLE.clone()).width(150).height(150),
        // ].into()
        canvas(self).height(Length::Fill).width(Length::Fill).into()

    }
}

impl <Message> canvas::Program<Message> for AnimatePenguin{
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
                    x: 200.0,
                    y: 200.0,
                    width: 150.0,
                    height: 150.0,
                },
                image,
            );
        });

        vec![screen]
    }
}