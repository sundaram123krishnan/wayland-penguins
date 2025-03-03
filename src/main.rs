use iced::{Color, Element, Renderer, Task, Theme};
use iced_layershell::{to_layer_message, Application};
use iced::widget::{text, column, button, image};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use std::sync::LazyLock;
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
struct AnimatePenguin {}

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
            Self {}, Task::none()
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
        column![
            text("hello"),
           image(PENGUIN_HANDLE.clone()).width(150).height(150),
        ].into()
    }
}
