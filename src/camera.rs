use macroquad::camera::{Camera2D, set_camera};
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::{screen_height, screen_width};

static mut _CAMERA: Option<Camera> = None;

#[allow(non_snake_case)]
pub(crate) fn CAMERA() -> &'static mut Camera {
    unsafe {
        if _CAMERA.is_none() { _CAMERA = Option::from(Camera::new()) }
        return _CAMERA.as_mut().unwrap()
    }
}

pub(crate) struct Camera {
    camera: Camera2D
}
impl Camera {
    pub fn new() -> Camera {
        Camera {
            camera: Camera2D {
                zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0),
                target: vec2(screen_width() / 2.0, screen_height() / 2.0),
                ..Default::default()
            }
        }
    }

    pub fn update_camera(&mut self) {
        set_camera(&self.camera);
    }

    pub fn set_target(&mut self, target: Vec2) {
        self.camera.target = target;
        self.update_camera();
    }
}
