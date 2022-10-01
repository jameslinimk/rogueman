use crate::camera::CAMERA;
use crate::scenes::objects::shapes::rect::Rect;
use crate::util::rel_mouse_pos;
use crate::{KeyCode, GAME};
use macroquad::color::{BLUE, WHITE};
use macroquad::input::is_key_down;
use macroquad::shapes::draw_line;
use macroquad::text::draw_text;
use macroquad::time::get_frame_time;
use macroquad::window::{screen_height, screen_width};

#[derive(Debug)]
pub(crate) struct Player {
    pub rect: Rect,
    speed: f32,
}
impl Player {
    pub fn new() -> Player {
        Player {
            rect: Rect::new_center(0.0, 0.0, 30.0, 30.0),
            speed: 500.0,
        }
    }
}
impl Player {
    pub fn update(&mut self) {
        let mut hspd = 0.0;
        let mut vspd = 0.0;

        if is_key_down(KeyCode::W) {
            vspd -= 1.0
        }
        if is_key_down(KeyCode::S) {
            vspd += 1.0
        }
        if is_key_down(KeyCode::A) {
            hspd -= 1.0
        }
        if is_key_down(KeyCode::D) {
            hspd += 1.0
        }

        // Band-aid patch for diagonal movement
        let dia = if hspd != 0.0 && vspd != 0.0 {
            0.707
        } else {
            1.0
        };

        let ft = get_frame_time();
        hspd *= self.speed * ft * dia;
        vspd *= self.speed * ft * dia;

        // Collision detection
        for wall in &GAME().walls {
            self.rect.pos.x += hspd;
            if self.rect.touches(wall) {
                if self.rect.pos.x > wall.pos.x {
                    self.rect.set_left(wall.get_right());
                } else {
                    self.rect.set_right(wall.get_left());
                }
            }

            self.rect.pos.y += vspd;
            if self.rect.touches(wall) {
                if self.rect.pos.y > wall.pos.y {
                    self.rect.set_top(wall.get_bottom());
                } else {
                    self.rect.set_bottom(wall.get_top());
                }
            }
        }

        CAMERA().target = self.rect.get_center();
    }

    pub fn draw(&mut self) {
        self.rect.draw(WHITE);

        let mouse_pos = rel_mouse_pos();

        // Extend line by length
        let length = screen_height() + screen_width();
        let alpha =
            (mouse_pos.1 - self.rect.get_center().y).atan2(mouse_pos.0 - self.rect.get_center().x);

        // ! If you move right enough the mouse line stops working (SOMETHING WRONG WITH RELATIVE COORDS)
        println!(
            "{:?} | {:?}",
            (self.rect.get_center().x, self.rect.get_center().y),
            (
                mouse_pos.0 + length * alpha.cos(),
                mouse_pos.1 + length * alpha.sin()
            )
        );

        draw_line(
            self.rect.get_center().x,
            self.rect.get_center().y,
            mouse_pos.0 + length * alpha.cos(),
            mouse_pos.1 + length * alpha.sin(),
            2.0,
            BLUE,
        );

        draw_text(
            &format!("xy: {:?}", self.rect.get_center()),
            0.0 - (screen_width() / 2.0 - CAMERA().camera.target.x).abs(),
            27.0 - (screen_height() / 2.0 - CAMERA().camera.target.y).abs(),
            50.0,
            WHITE,
        );
    }
}
