use macroquad::color::{BLUE, WHITE};
use macroquad::input::{is_key_down, mouse_position};
use macroquad::shapes::draw_line;
use macroquad::time::get_frame_time;
use macroquad::window::{screen_height, screen_width};
use crate::{GameScene, KeyCode, Object};
use crate::scenes::objects::shapes::rect::Rect;

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
impl Player {
    pub fn update(&mut self) {
        let mut hspd = 0_f32;
        let mut vspd = 0_f32;

        if is_key_down(KeyCode::W) { vspd -= 1.0 }
        if is_key_down(KeyCode::S) { vspd += 1.0 }
        if is_key_down(KeyCode::A) { hspd -= 1.0 }
        if is_key_down(KeyCode::D) { hspd += 1.0 }

        // Band-aid patch for diagonal movement
        let dia = if hspd != 0_f32 && vspd != 0_f32 { 0.707_f32 } else { 1.0 };

        self.rect.pos.x += hspd * self.speed * get_frame_time() * 10.0 * dia;
        self.rect.pos.y += vspd * self.speed * get_frame_time() * 10.0 * dia;
    }

    pub fn draw(&mut self) {
        self.rect.draw(WHITE);

        let mouse_pos = mouse_position();

        // Extend line by length
        let length = screen_height() + screen_width();
        let alpha = (mouse_pos.1 - self.rect.get_center().y).atan2(mouse_pos.0 - self.rect.get_center().x);

        draw_line(
            self.rect.get_center().x,
            self.rect.get_center().y,
            mouse_pos.0 + length * alpha.cos(),
            mouse_pos.1 + length * alpha.sin(),
            2_f32,
            BLUE
        );
    }
}
