use std::f32::consts::PI;

use macroquad::{
    prelude::{mouse_position, vec2, Vec2},
    window::{screen_height, screen_width},
};

use crate::camera::CAMERA;

pub(crate) fn angle(origin: Vec2, dest: Vec2) -> f32 {
    let x_dist = dest.x - origin.x;
    let y_dist = dest.y - origin.y;

    return (-y_dist).atan2(x_dist) % (2.0 * PI);
}

pub(crate) fn opposite_angle(origin: Vec2, dest: Vec2) -> f32 {
    let x_dist = origin.x - dest.x;
    let y_dist = origin.y - dest.y;

    return (-y_dist).atan2(x_dist) % (2.0 * PI);
}

pub(crate) fn project(origin: Vec2, angle: f32, distance: f32) -> Vec2 {
    return vec2(
        origin.x + (angle.cos() * distance),
        origin.y - (angle.sin() * distance),
    );
}

/// Adjusted mouse position for the current camera
pub(crate) fn adj_mouse_pos() -> (f32, f32) {
    let mouse_pos = mouse_position();
    return (
        mouse_pos.0 - (screen_width() / 2.0 - CAMERA().camera.target.x).abs(),
        mouse_pos.1 - (screen_height() / 2.0 - CAMERA().camera.target.y).abs(),
    );
}

pub(crate) fn distance(p1: Vec2, p2: Vec2) -> f32 {
    return ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).sqrt();
}

pub(crate) fn easeInOut(value: f32) -> {
    // return x === 0
    // ? 0
    // : x === 1
    // ? 1
    // : x < 0.5 ? Math.pow(2, 20 * x - 10) / 2
    // : (2 - Math.pow(2, -20 * x + 10)) / 2;
}
