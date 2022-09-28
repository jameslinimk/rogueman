use std::borrow::BorrowMut;
use macroquad::math::{Vec2, vec2};

static mut CAMERA: Option<Camera> = None;

pub(crate) unsafe fn get_camera() -> &'static mut Camera {
    if CAMERA.is_none() { CAMERA = Option::from(Camera::new()) }
    return CAMERA.as_mut().unwrap()
}

pub(crate) struct Camera {
    follow: Vec2
}
impl Camera {
    pub fn new() -> Camera {
        Camera {
            follow: vec2(0.0, 0.0)
        }
    }

    pub fn set_follow(&mut self, follow: Vec2) {
        self.follow = follow;
    }
}
