use macroquad::camera::{set_camera, Camera2D};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{screen_height, screen_width};
use macroquad::rand::gen_range;
use macroquad::time::{get_frame_time, get_time};

use crate::pub_global_variable;
use crate::util::{angle, distance, ease_in_out, project};

pub_global_variable!(CAMERA, _CAMERA, Camera);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ShakeConfig {
    pub duration: f32,
    pub intensity: f32,
}

pub(crate) struct Camera {
    pub camera: Camera2D,
    pub target: Vec2,
    pub shake: Option<ShakeConfig>,
    pub shake_offset: Vec2,
    pub shake_start: f64,
}
impl Camera {
    pub fn new() -> Camera {
        Camera {
            camera: Camera2D {
                zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0),
                target: vec2(screen_width() / 2.0, screen_height() / 2.0),
                ..Default::default()
            },
            target: vec2(screen_width() / 2.0, screen_height() / 2.0),
            shake: None,
            shake_offset: vec2(0.0, 0.0),
            shake_start: 0.0,
        }
    }

    pub fn update_camera(&mut self) {
        set_camera(&self.camera);
    }

    pub fn update(&mut self) {
        let dis = distance(self.camera.target, self.target);
        let max_increase = screen_width().max(screen_height()) / 2.0;

        let ratio = ease_in_out(dis / max_increase);

        let pan_speed = (2000.0 * ratio) * get_frame_time();

        if dis > pan_speed {
            let angle = angle(self.camera.target, self.target);
            self.camera.target = project(self.camera.target, angle, pan_speed);
            self.update_camera();
        }

        if self.shake.is_some()
            && (self.shake_start == 0.0
                || get_time() > self.shake_start + self.shake.as_mut().unwrap().duration as f64)
        {
            self.shake = None;
        }

        if self.shake.is_some() {
            let intense = -self.shake.as_mut().unwrap().intensity * get_frame_time();

            self.shake_offset.x = gen_range(-intense, intense);
            self.shake_offset.y = gen_range(-intense, intense);

            self.camera.target.x += self.shake_offset.x;
            self.camera.target.y += self.shake_offset.y;
        }
    }

    pub fn set_shake(&mut self, shake: ShakeConfig) {
        self.shake = Option::from(shake);
        self.shake_start = get_time();
    }

    pub fn remove_shake(&mut self) {
        self.shake = None;
    }
}
