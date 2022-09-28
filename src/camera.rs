use std::borrow::BorrowMut;
use macroquad::math::{Vec2, vec2};

static mut _CAMERA: Option<Camera> = None;

pub(crate) fn CAMERA() -> &'static mut Camera {
    unsafe {
        if _CAMERA.is_none() { _CAMERA = Option::from(Camera::new()) }
        return _CAMERA.as_mut().unwrap()
    }
}

pub(crate) struct Camera {
    pub(crate) follow: Vec2
}
impl Camera {
    pub fn new() -> Camera {
        Camera {
            follow: vec2(0.0, 0.0)
        }
    }

    pub fn fx(&self, x: f32) -> f32 {
        return x - self.follow.x
    }

    pub fn fy(&self, y: f32) -> f32 {
        return y - self.follow.y
    }
}
