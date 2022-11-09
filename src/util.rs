use core::panic;
use std::f32::consts::PI;

use macroquad::prelude::{
    draw_text, draw_text_ex, measure_text, mouse_position, screen_height, screen_width, vec2,
    Color, KeyCode, TextParams, Vec2,
};
use macroquad::rand::gen_range;

use crate::scenes::game::GAME;

pub const NUMBER_KEYS: [KeyCode; 4] = [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4];
pub const DAMAGE_COOLDOWN: f64 = 0.25;
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    W,
    A,
    S,
    D,
    WA,
    WD,
    SA,
    SD,
}
pub const DIRECTIONS: [(Direction, &str); 8] = [
    (Direction::W, "w"),
    (Direction::A, "a"),
    (Direction::S, "s"),
    (Direction::D, "d"),
    (Direction::WA, "wa"),
    (Direction::WD, "wd"),
    (Direction::SA, "sa"),
    (Direction::SD, "sd"),
];
pub const SQUARE_SIZE: f32 = 30.0;

/// It takes two points, and returns the angle between them
pub fn angle(origin: Vec2, dest: Vec2) -> f32 {
    let x_dist = dest.x - origin.x;
    let y_dist = dest.y - origin.y;

    (-y_dist).atan2(x_dist) % (2.0 * PI)
}

/// It takes two points, and returns the opposite angle between them
pub fn opposite_angle(origin: Vec2, dest: Vec2) -> f32 {
    let x_dist = origin.x - dest.x;
    let y_dist = origin.y - dest.y;

    (-y_dist).atan2(x_dist) % (2.0 * PI)
}

/// It takes a point, an angle, and a distance, and returns a new point that is the distance away from the original point in the direction of the angle
pub fn project(origin: Vec2, angle: f32, distance: f32) -> Vec2 {
    vec2(
        origin.x + (angle.cos() * distance),
        origin.y - (angle.sin() * distance),
    )
}

/// Converts degrees to radians
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

/// Converts radians to degrees
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / PI
}

/// It returns the x position relative to the screen
pub fn rx(x: f32) -> f32 {
    let shake_offset = if GAME().camera.shake.is_none() {
        0.0
    } else {
        GAME().camera.shake_offset.x
    };
    return x - (screen_width() / 2.0 - GAME().camera.camera.target.x - shake_offset);
}

/// It returns the y position relative to the screen
pub fn ry(y: f32) -> f32 {
    let shake_offset = if GAME().camera.shake.is_none() {
        0.0
    } else {
        GAME().camera.shake_offset.y
    };
    return y - (screen_height() / 2.0 - GAME().camera.camera.target.y - shake_offset);
}

/// It returns the x position relative to the screen (counteracted to adjust for shake)
pub fn rx_smooth(x: f32) -> f32 {
    let shake_offset = if GAME().camera.shake.is_none() {
        0.0
    } else {
        GAME().camera.shake_offset.x
    };
    return x - (screen_width() / 2.0 - GAME().camera.camera.target.x + shake_offset);
}

/// It returns the y position relative to the screen (counteracted to adjust for shake)
pub fn ry_smooth(y: f32) -> f32 {
    let shake_offset = if GAME().camera.shake.is_none() {
        0.0
    } else {
        GAME().camera.shake_offset.y
    };
    return y - (screen_height() / 2.0 - GAME().camera.camera.target.y + shake_offset);
}

/// It returns the mouse position relative to the screen
pub fn rel_mouse_pos() -> Vec2 {
    let mouse_pos = mouse_position();
    vec2(rx(mouse_pos.0), ry(mouse_pos.1))
}

/// It takes two points and returns the distance between them
pub fn distance(p1: Vec2, p2: Vec2) -> f32 {
    ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).sqrt()
}

/// Converts a value from `0.0` - `1.0` to an ease-in-out curve (sign wave)
pub fn ease_in_out(x: f32) -> f32 {
    (-((PI * x).cos() - 1.0) / 2.0).clamp(0.0, 1.0)
}

pub fn multiline_text(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
    let height = measure_text(text, None, font_size, 1.0).height;
    for (i, line) in text.lines().enumerate() {
        draw_text(line, x, y + height * i as f32, font_size as f32, color);
    }
}

pub fn multiline_text_ex(text: &str, x: f32, y: f32, font_size: u16, params: TextParams) {
    let height = measure_text(
        text,
        Option::from(params.font),
        font_size,
        params.font_scale,
    )
    .height;
    for (i, line) in text.lines().enumerate() {
        draw_text_ex(line, x, y + height * i as f32, params);
    }
}

#[macro_export]
/// Crate a global (a static mut) and a public getter function so it can be used safely
macro_rules! pub_global_variable {
    ($name: ident, $raw_name: ident, $b: ty) => {
        static mut $raw_name: Option<$b> = None;

        #[allow(non_snake_case)]
        pub fn $name() -> &'static mut $b {
            unsafe {
                if $raw_name.is_none() {
                    $raw_name = Option::from(<$b>::new())
                }
                return $raw_name.as_mut().unwrap();
            }
        }
    };
}

#[macro_export]
macro_rules! game_remove {
    ($vector: expr, $id: expr) => {
        match $vector.iter().position(|x| x.get_id() == $id) {
            Some(index) => {
                $vector.remove(index);
            }
            None => {}
        };
    };
}

#[macro_export]
macro_rules! repeat_for_vec {
    ( $function: ident, $( $vector: expr ), * ) => {
        $(
            for x in &mut $vector {
                x.$function();
            }
        )*
    };
}

#[macro_export]
macro_rules! repeat_function {
    ( $function: ident, $( $value: expr ), * ) => {
        $(
            $value.$function();
        )*
    };
}

#[macro_export]
macro_rules! unwrap_or_return {
    ( $option: expr ) => {
        match $option {
            Some(value) => value,
            None => return,
        }
    };
}

pub fn hex(hex: &'static str) -> Color {
    Color::from_rgba(
        u8::from_str_radix(&hex[1..3], 16).unwrap(),
        u8::from_str_radix(&hex[3..5], 16).unwrap(),
        u8::from_str_radix(&hex[5..7], 16).unwrap(),
        255,
    )
}

pub fn random_array<T>(array: &[T]) -> &T {
    if array.is_empty() {
        panic!("Array is empty");
    }
    &array[gen_range(0, array.len())]
}
