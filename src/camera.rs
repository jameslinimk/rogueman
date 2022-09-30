use macroquad::camera::{set_camera, Camera2D};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{screen_height, screen_width};
use macroquad::time::get_frame_time;

use crate::util::{angle, distance, ease_in_out, project};

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

    pub fn update_camera(&mut self) {
        set_camera(&self.camera);
    }

    pub fn set_target(&mut self, target: Vec2) {
        self.go_to = target;
    }

    pub fn update(&mut self) {
        let dis = distance(self.camera.target, self.go_to);
        // let max_increase = (screen_width().powf(2.0) + screen_height().powf(2.0)).sqrt() / 2.0;
        let max_increase = screen_width().max(screen_height()) / 2.0;
        let ratio = ease_in_out(dis / max_increase);

        let pan_speed = (2000.0 * ratio) * get_frame_time();

        if dis > pan_speed {
            let angle = angle(self.camera.target, self.go_to);
            self.camera.target = project(self.camera.target, angle, pan_speed);
            self.update_camera();
        }
    }
}
