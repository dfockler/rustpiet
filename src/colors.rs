use image::{Pixel, Rgba};

pub fn color_code(color: &Rgba<u8>) -> Option<i32> {
    match color.channels4() {
        (255, 192, 192, _) => Some(11),
        (255, 255, 192, _) => Some(21),
        (192, 255, 192, _) => Some(31),
        (192, 255, 255, _) => Some(41),
        (192, 192, 255, _) => Some(51),
        (255, 192, 255, _) => Some(61),
        (255, 0, 0, _) => Some(12),
        (255, 255, 0, _) => Some(22),
        (0, 255, 0, _) => Some(32),
        (0, 255, 255, _) => Some(42),
        (0, 0, 255, _) => Some(52),
        (255, 0, 255, _) => Some(62),
        (192, 0, 0, _) => Some(13),
        (192, 192, 0, _) => Some(23),
        (0, 192, 0, _) => Some(33),
        (0, 192, 192, _) => Some(43),
        (0, 0, 192, _) => Some(53),
        (192, 0, 192, _) => Some(63),
        (255, 255, 255, _) => Some(1),
        (0, 0, 0, _) => Some(0),
        _ => None,
    }
}

// 6 hues (Red, Yellow, Green, Cyan, Blue, Magenta)
pub fn hue_difference(first: i32, second: i32) -> i32 {
    let first_hue = first / 10;
    let second_hue = second / 10;
    if first_hue > second_hue {
        (first_hue - (second_hue + 6)).abs()
    } else {
        second_hue - first_hue
    }
}

// 3 shades (Light, Medium, Dark)
pub fn shade_difference(first: i32, second: i32) -> i32 {
    let first_shade = first % 10;
    let second_shade = second % 10;
    if first_shade > second_shade {
        (first_shade - (second_shade + 3)).abs()
    } else {
        second_shade - first_shade
    }
}
