use iced::widget::image;
use std::sync::LazyLock;

const PENGUIN: &[u8] = include_bytes!("../assets/pngwing.com.png");
pub static PENGUIN_HANDLE: LazyLock<image::Handle> =
    LazyLock::new(|| image::Handle::from_bytes(PENGUIN));

const PENGUIN1: &[u8] = include_bytes!("../assets/linux.png");
pub static PENGUIN1_HANDLE: LazyLock<image::Handle> =
    LazyLock::new(|| image::Handle::from_bytes(PENGUIN1));
