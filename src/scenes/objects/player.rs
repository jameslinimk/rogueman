use macroquad::color::{BLUE, WHITE};
use macroquad::input::{is_key_down, mouse_position};
use macroquad::shapes::draw_line;
use macroquad::time::get_frame_time;
use macroquad::window::{screen_height, screen_width};
use crate::{GAME, KeyCode};
use crate::scenes::objects::shapes::rect::Rect;

#[derive(Debug)]
pub(crate) struct Player {
    rect: Rect,
    speed: f32
}
impl Player {
    pub fn new() -> Player {
        Player {
            rect: Rect::new_center(0.0, 0.0, 30.0, 30.0),
            speed: 2.5
        }
    }
}
impl Player {
    pub fn update(&mut self) {
        let mut hspd = 0.0;
        let mut vspd = 0.0;

        if is_key_down(KeyCode::W) { vspd -= 1.0 }
        if is_key_down(KeyCode::S) { vspd += 1.0 }
        if is_key_down(KeyCode::A) { hspd -= 1.0 }
        if is_key_down(KeyCode::D) { hspd += 1.0 }

        // Band-aid patch for diagonal movement
        let dia = if hspd != 0.0 && vspd != 0.0 { 0.707 } else { 1.0 };

        hspd *= self.speed * get_frame_time() * 100.0 * dia;
        vspd *= self.speed * get_frame_time() * 100.0 * dia;

        // Collision detection
        let og_pos = self.rect.pos;

        self.rect.pos.x += hspd;
        self.rect.pos.y += vspd;

        for wall in &GAME().walls { if self.rect.touches(wall) {
            self.rect.pos = og_pos;
        } }
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
            2.0,
            BLUE
        )
    }
}
