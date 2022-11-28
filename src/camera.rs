use derive_new::new;
use macroquad::prelude::rand::gen_range;
use macroquad::prelude::{
    get_frame_time, get_time, screen_height, screen_width, set_camera, vec2, Camera2D, Vec2,
};

use crate::util::{angle, distance, ease_in_out, project};

#[derive(Debug, Clone, Copy)]
pub struct ShakeConfig {
    pub duration: f32,
    pub intensity: f32,
}

#[derive(new)]
pub struct Camera {
    #[new(value = "Camera2D {
        zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0),
        target: vec2(screen_width() / 2.0, screen_height() / 2.0),
        ..Default::default()
    }")]
    pub camera: Camera2D,
    #[new(value = "vec2(screen_width() / 2.0, screen_height() / 2.0)")]
    pub target: Vec2,
    #[new(value = "None")]
    pub shake: Option<ShakeConfig>,
    #[new(value = "vec2(0.0, 0.0)")]
    pub shake_offset: Vec2,
    #[new(value = "0.0")]
    pub shake_start: f64,
}
impl Camera {
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
        self.shake = Some(shake);
        self.shake_start = get_time();
    }

    pub fn remove_shake(&mut self) {
        self.shake = None;
    }
}
