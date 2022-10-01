use std::f32::consts::PI;

use macroquad::{
    prelude::{mouse_position, vec2, Vec2},
    window::{screen_height, screen_width},
};

use crate::camera::CAMERA;

/// It takes two points, and returns the angle between them
///
/// Arguments:
///
/// * `origin`: The origin of the angle.
/// * `dest`: The destination point.
///
/// Returns:
///
/// The angle between the two points.
pub(crate) fn angle(origin: Vec2, dest: Vec2) -> f32 {
    let x_dist = dest.x - origin.x;
    let y_dist = dest.y - origin.y;

    return (-y_dist).atan2(x_dist) % (2.0 * PI);
}

/// It takes two points, and returns the angle from the first point to the second point
///
/// Arguments:
///
/// * `origin`: The origin of the angle.
/// * `dest`: The destination point.
///
/// Returns:
///
/// The angle between the two points.
pub(crate) fn opposite_angle(origin: Vec2, dest: Vec2) -> f32 {
    let x_dist = origin.x - dest.x;
    let y_dist = origin.y - dest.y;

    return (-y_dist).atan2(x_dist) % (2.0 * PI);
}

/// It takes a point, an angle, and a distance, and returns a new point that is the distance away from
/// the original point in the direction of the angle
///
/// Arguments:
///
/// * `origin`: The origin of the ray.
/// * `angle`: The angle of the ray in radians.
/// * `distance`: The distance from the origin to the point.
///
/// Returns:
///
/// A Vec2
pub(crate) fn project(origin: Vec2, angle: f32, distance: f32) -> Vec2 {
    return vec2(
        origin.x + (angle.cos() * distance),
        origin.y - (angle.sin() * distance),
    );
}

/// It returns the mouse position relative to the camera's position
pub(crate) fn rel_mouse_pos() -> (f32, f32) {
    let mouse_pos = mouse_position();
    return (
        mouse_pos.0 - (screen_width() / 2.0 - CAMERA().camera.target.x).abs(),
        mouse_pos.1 - (screen_height() / 2.0 - CAMERA().camera.target.y).abs(),
    );
}

/// It takes two points and returns the distance between them
///
/// Arguments:
///
/// * `p1`: The first point
/// * `p2`: Vec2 - The point to check the distance to.
///
/// Returns:
///
/// A function that takes two Vec2s and returns a f32.
pub(crate) fn distance(p1: Vec2, p2: Vec2) -> f32 {
    return ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).sqrt();
}

/// Converts a value from 0-1 to an ease-in-out curve (sign wave)
///
/// Arguments:
///
/// * `x`: A value between 0-1.
///
/// Returns:
///
/// A value between 0 and 1 on the ease in scale.
pub(crate) fn ease_in_out(x: f32) -> f32 {
    return (-((PI * x).cos() - 1.0) / 2.0).clamp(0.0, 1.0);
}

#[macro_export]
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
