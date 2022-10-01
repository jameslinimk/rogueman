use std::f32::consts::PI;

use macroquad::{
    prelude::{mouse_position, vec2, Vec2},
    window::{screen_height, screen_width},
};

use crate::camera::CAMERA;

/// It takes two points, and returns the angle between them
pub(crate) fn angle(origin: Vec2, dest: Vec2) -> f32 {
    let x_dist = dest.x - origin.x;
    let y_dist = dest.y - origin.y;

    return (-y_dist).atan2(x_dist) % (2.0 * PI);
}

/// It takes two points, and returns the opposite angle between them
pub(crate) fn opposite_angle(origin: Vec2, dest: Vec2) -> f32 {
    let x_dist = origin.x - dest.x;
    let y_dist = origin.y - dest.y;

    return (-y_dist).atan2(x_dist) % (2.0 * PI);
}

/// It takes a point, an angle, and a distance, and returns a new point that is the distance away from the original point in the direction of the angle
pub(crate) fn project(origin: Vec2, angle: f32, distance: f32) -> Vec2 {
    return vec2(
        origin.x + (angle.cos() * distance),
        origin.y - (angle.sin() * distance),
    );
}

/// It returns the x position relative to the screen
pub(crate) fn rx(x: f32) -> f32 {
    return x - (screen_width() / 2.0 - CAMERA().camera.target.x);
}

/// It returns the x position relative to the screen
pub(crate) fn ry(y: f32) -> f32 {
    return y - (screen_height() / 2.0 - CAMERA().camera.target.y);
}

/// It returns the mouse position relative to the screen
pub(crate) fn rel_mouse_pos() -> Vec2 {
    let mouse_pos = mouse_position();
    return vec2(rx(mouse_pos.0), ry(mouse_pos.1));
}

/// It takes two points and returns the distance between them
pub(crate) fn distance(p1: Vec2, p2: Vec2) -> f32 {
    return ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).sqrt();
}

/// Converts a value from `0.0` - `1.0` to an ease-in-out curve (sign wave)
pub(crate) fn ease_in_out(x: f32) -> f32 {
    return (-((PI * x).cos() - 1.0) / 2.0).clamp(0.0, 1.0);
}

#[macro_export]
/// Crate a global (a static mut) and a public getter function so it can be used safely
macro_rules! pub_global_variable {
    ($name: ident, $raw_name: ident, $b: ty) => {
        static mut $raw_name: Option<$b> = None;

        #[allow(non_snake_case)]
        pub(crate) fn $name() -> &'static mut $b {
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
/// Crate a global (a static mut) and a getter function so it can be used safely
macro_rules! global_variable {
    ($name: ident, $raw_name: ident, $b: ty) => {
        static mut $raw_name: Option<$b> = None;

        #[allow(non_snake_case)]
        fn $name() -> &'static mut $b {
            unsafe {
                if $raw_name.is_none() {
                    $raw_name = Option::from(<$b>::new())
                }
                return $raw_name.as_mut().unwrap();
            }
        }
    };
}
