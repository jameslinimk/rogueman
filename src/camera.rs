use macroquad::camera::{set_camera, Camera2D};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{mouse_position, screen_height, screen_width};
use macroquad::time::get_frame_time;

static mut _CAMERA: Option<Camera> = None;

#[allow(non_snake_case)]
pub(crate) fn CAMERA() -> &'static mut Camera {
    unsafe {
        if _CAMERA.is_none() {
            _CAMERA = Option::from(Camera::new())
        }
        return _CAMERA.as_mut().unwrap();
    }
}

/// Adjusted mouse position for the current camera
pub(crate) fn adj_mouse_pos() -> (f32, f32) {
    let mouse_pos = mouse_position();
    return (
        mouse_pos.0 - (screen_width() / 2.0 - CAMERA().camera.target.x).abs(),
        mouse_pos.1 - (screen_height() / 2.0 - CAMERA().camera.target.y).abs(),
    );
}

pub(crate) struct Camera {
    pub camera: Camera2D,
    pub go_to: Vec2,
}
impl Camera {
    pub fn new() -> Camera {
        Camera {
            camera: Camera2D {
                zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0),
                target: vec2(screen_width() / 2.0, screen_height() / 2.0),
                ..Default::default()
            },
            go_to: vec2(screen_width() / 2.0, screen_height() / 2.0),
        }
    }

    pub fn init_camera(&mut self) {
        set_camera(&self.camera);
    }

    pub fn set_target(&mut self, target: Vec2) {
        self.go_to = target;
    }

    pub fn update(&mut self) {
        let pan_speed = 5.0 * get_frame_time();
        // Move to pan_speed
    }
}
