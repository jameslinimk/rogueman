use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::draw_rectangle;
use crate::{vec2};

#[derive(Debug, Copy, Clone)]
pub (crate) struct Rect {
    pub(crate) pos: Vec2,
    pub(crate) width: f32,
    pub(crate) height: f32
}
impl Rect {
    /// Create a new rect from top-left point
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            pos: vec2(x, y),
            width,
            height
        }
    }

    /// Creates new rect from a center point
    pub fn new_center(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            pos: vec2(x - width / 2.0, y - width / 2.0),
            width,
            height
        }
    }

    pub fn get_center(&self) -> Vec2 {
        return vec2(self.pos.x + self.width / 2.0, self.pos.y + self.height / 2.0);
    }

    pub fn set_center(&mut self, x: f32, y: f32) {
        self.pos.x = x - self.width / 2.0;
        self.pos.y = y - self.height / 2.0;
    }

    pub fn get_bottom_right(&self) -> Vec2 {
        return vec2(self.pos.x + self.width, self.pos.y + self.height)
    }

    pub fn touches(&self, rect: &Rect) -> bool {
        /*
        println!("1: {} {} {}", self.pos.x <= rect.get_bottom_right().x, self.pos.x, rect.get_bottom_right().x);
        println!();
        println!("2: {} {} {}", self.get_bottom_right().x >= rect.pos.x, self.get_bottom_right().x, rect.pos.y);
        println!();
        println!("3: {} {} {}", self.pos.y >= rect.get_bottom_right().y, self.pos.y, rect.get_bottom_right().y);
        println!();
        println!("4: {} {} {}", self.get_bottom_right().y <= rect.pos.y, self.get_bottom_right().y, rect.pos.y);

        println!("\n{:?}\n{:?}", self, rect);
        if touches {
            println!("TOUCHES!");
        }
        */

        return
            self.pos.x <= rect.get_bottom_right().x &&
            self.get_bottom_right().x >= rect.pos.x &&
            self.pos.y <= rect.get_bottom_right().y &&
            self.get_bottom_right().y >= rect.pos.y;
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, color);
    }
}
