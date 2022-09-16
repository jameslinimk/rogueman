use macroquad::color::WHITE;
use macroquad::input::is_key_down;
use macroquad::time::get_frame_time;
use crate::{KeyCode, Object};
use crate::scenes::objects::rect::Rect;

pub(crate) struct Player {
    rect: Rect,
    speed: f32
}
impl Player {
    pub fn new() -> Player {
        Player {
            rect: Rect::new_center(0_f32, 0_f32, 30_f32, 30_f32),
            speed: 25_f32
        }
    }
}
impl Object for Player {
    fn update(&mut self) {
        let mut hspd = 0_f32;
        let mut vspd = 0_f32;

        if is_key_down(KeyCode::W) { vspd -= 1.0 }
        if is_key_down(KeyCode::S) { vspd += 1.0 }
        if is_key_down(KeyCode::A) { hspd -= 1.0 }
        if is_key_down(KeyCode::D) { hspd += 1.0 }

        self.rect.pos.x += hspd * self.speed * get_frame_time() * 10.0;
        self.rect.pos.y += vspd * self.speed * get_frame_time() * 10.0;
    }

    fn draw(&mut self) {
        self.rect.draw(WHITE);
    }
}
