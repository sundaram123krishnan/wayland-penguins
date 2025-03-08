use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, column, image};
use iced::{
    Color, Element, Length, Point, Radians, Rectangle, Renderer, Size, Task, Theme,
};
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::{to_layer_message, Application};
use std::process::Command;
use std::sync::LazyLock;

const PENGUIN: &[u8] = include_bytes!("../assets/pngwing.com.png");
static PENGUIN_HANDLE: LazyLock<image::Handle> =
    LazyLock::new(|| image::Handle::from_bytes(PENGUIN));

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("xdpyinfo | grep dimensions")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let (width, height) = stdout
        .split_whitespace()
        .nth(1)
        .and_then(|dimensions| {
            let parts: Vec<&str> = dimensions.split('x').collect();
            if parts.len() == 2 {
                Some((parts[0].parse::<u32>().ok()?, parts[1].parse::<u32>().ok()?))
            } else {
                None
            }
        })
        .expect("Failed to parse screen dimensions");

    AnimatePenguin::run(Settings {
        flags: (width, height),
        layer_settings: LayerShellSettings {
            size: Some((width, height)),
            events_transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
}
#[derive(Default)]
struct AnimatePenguin {
    draw_cache: Cache,
    move_x: f32,
    move_y: f32,
    screen_size: (u32, u32),
    moving_right: bool,
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
    type Flags = (u32, u32);

    fn new(flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let bottom = flags.1 as f32 - 50.0f32;
        (
            Self {
                screen_size: flags,
                move_y: bottom,
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
        iced::time::every(std::time::Duration::from_millis(10))
            .map(|_| Message::Tick)
    }

    fn namespace(&self) -> String {
        String::from("Penguins Animation")
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        return match message {
            Message::Tick => {
                let max_x = self.screen_size.0 as f32 - 60.0f32;
                if self.move_x >= max_x {
                    self.moving_right = false;
                } else if self.move_x <= 0.0 {
                    self.moving_right = true;
                }

                if self.moving_right {
                    self.move_x += 2.0; 
                } else {
                    self.move_x -= 2.0;
                }
                println!("x: {}", self.move_x);
                self.draw_cache.clear();
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
                    y: self.move_y,
                    width: 50.0,
                    height: 50.0,
                },
                image,
            );
        });

        vec![screen]
    }
}
