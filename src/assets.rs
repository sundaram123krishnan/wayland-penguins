use iced::widget::image;
use std::sync::LazyLock;

const PENGUIN0: &[u8] = include_bytes!("../assets/0.png");
const PENGUIN1: &[u8] = include_bytes!("../assets/01.png");
const PENGUIN2: &[u8] = include_bytes!("../assets/02.png");
const PENGUIN3: &[u8] = include_bytes!("../assets/03.png");
const PENGUIN4: &[u8] = include_bytes!("../assets/04.png");
const PENGUIN5: &[u8] = include_bytes!("../assets/05.png");
const PENGUIN6: &[u8] = include_bytes!("../assets/06.png");
const PENGUIN7: &[u8] = include_bytes!("../assets/07.png");
const PENGUIN8: &[u8] = include_bytes!("../assets/08.png");
const PENGUIN9: &[u8] = include_bytes!("../assets/09.png");
const PENGUIN10: &[u8] = include_bytes!("../assets/10.png");
const PENGUIN11: &[u8] = include_bytes!("../assets/11.png");
const PENGUIN12: &[u8] = include_bytes!("../assets/12.png");
const PENGUIN13: &[u8] = include_bytes!("../assets/13.png");
const PENGUIN14: &[u8] = include_bytes!("../assets/14.png");
const PENGUIN15: &[u8] = include_bytes!("../assets/15.png");
const PENGUIN16: &[u8] = include_bytes!("../assets/16.png");
const PENGUIN17: &[u8] = include_bytes!("../assets/17.png");
const PENGUIN18: &[u8] = include_bytes!("../assets/18.png");
const PENGUIN19: &[u8] = include_bytes!("../assets/19.png");
const PENGUIN20: &[u8] = include_bytes!("../assets/29.png");
const PENGUIN21: &[u8] = include_bytes!("../assets/21.png");
const PENGUIN22: &[u8] = include_bytes!("../assets/22.png");
const PENGUIN23: &[u8] = include_bytes!("../assets/23.png");
const PENGUIN24: &[u8] = include_bytes!("../assets/24.png");
const PENGUIN25: &[u8] = include_bytes!("../assets/25.png");
const PENGUIN26: &[u8] = include_bytes!("../assets/26.png");
const PENGUIN27: &[u8] = include_bytes!("../assets/27.png");
const PENGUIN28: &[u8] = include_bytes!("../assets/28.png");
const PENGUIN29: &[u8] = include_bytes!("../assets/29.png");
const PENGUIN30: &[u8] = include_bytes!("../assets/30.png");
const PENGUIN31: &[u8] = include_bytes!("../assets/31.png");
const PENGUIN32: &[u8] = include_bytes!("../assets/32.png");
const PENGUIN33: &[u8] = include_bytes!("../assets/33.png");
const PENGUIN34: &[u8] = include_bytes!("../assets/34.png");
const PENGUIN35: &[u8] = include_bytes!("../assets/35.png");
const PENGUIN36: &[u8] = include_bytes!("../assets/36.png");
const PENGUIN37: &[u8] = include_bytes!("../assets/37.png");
const PENGUIN38: &[u8] = include_bytes!("../assets/38.png");
const PENGUIN39: &[u8] = include_bytes!("../assets/39.png");
const PENGUIN40: &[u8] = include_bytes!("../assets/40.png");

pub static PENGUIN_HANDLES: LazyLock<Vec<image::Handle>> = LazyLock::new(|| {
    vec![
        image::Handle::from_bytes(PENGUIN0),
        image::Handle::from_bytes(PENGUIN1),
        image::Handle::from_bytes(PENGUIN2),
        image::Handle::from_bytes(PENGUIN3),
        image::Handle::from_bytes(PENGUIN4),
        image::Handle::from_bytes(PENGUIN5),
        image::Handle::from_bytes(PENGUIN6),
        image::Handle::from_bytes(PENGUIN7),
        image::Handle::from_bytes(PENGUIN8),
        image::Handle::from_bytes(PENGUIN9),
        image::Handle::from_bytes(PENGUIN10),
        image::Handle::from_bytes(PENGUIN11),
        image::Handle::from_bytes(PENGUIN12),
        image::Handle::from_bytes(PENGUIN13),
        image::Handle::from_bytes(PENGUIN14),
        image::Handle::from_bytes(PENGUIN15),
        image::Handle::from_bytes(PENGUIN16),
        image::Handle::from_bytes(PENGUIN17),
        image::Handle::from_bytes(PENGUIN18),
        image::Handle::from_bytes(PENGUIN19),
        image::Handle::from_bytes(PENGUIN20),
        image::Handle::from_bytes(PENGUIN21),
        image::Handle::from_bytes(PENGUIN22),
        image::Handle::from_bytes(PENGUIN23),
        image::Handle::from_bytes(PENGUIN24),
        image::Handle::from_bytes(PENGUIN25),
        image::Handle::from_bytes(PENGUIN26),
        image::Handle::from_bytes(PENGUIN27),
        image::Handle::from_bytes(PENGUIN28),
        image::Handle::from_bytes(PENGUIN29),
        image::Handle::from_bytes(PENGUIN30),
        image::Handle::from_bytes(PENGUIN31),
        image::Handle::from_bytes(PENGUIN32),
        image::Handle::from_bytes(PENGUIN33),
        image::Handle::from_bytes(PENGUIN34),
        image::Handle::from_bytes(PENGUIN35),
        image::Handle::from_bytes(PENGUIN36),
        image::Handle::from_bytes(PENGUIN37),
        image::Handle::from_bytes(PENGUIN38),
        image::Handle::from_bytes(PENGUIN39),
        image::Handle::from_bytes(PENGUIN40),
    ]
});

pub fn get_penguin_image(frame: usize) -> image::Handle {
    if frame >= PENGUIN_HANDLES.len()  {
        return PENGUIN_HANDLES[0].clone();
    }
    PENGUIN_HANDLES[frame].clone()
}
 