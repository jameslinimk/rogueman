use macroquad::prelude::{draw_rectangle, vec2, Color, Vec2};

use super::line::Line;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
}
impl Rect {
    /// Create a new rect from top-left point
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            pos: vec2(x, y),
            width,
            height,
        }
    }

    /// Creates new rect from a center point
    pub fn new_center(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            pos: vec2(x - width / 2.0, y - width / 2.0),
            width,
            height,
        }
    }
    pub fn new_center_vec(center: Vec2, width: f32, height: f32) -> Rect {
        Rect::new_center(center.x, center.y, width, height)
    }

    pub fn get_center(&self) -> Vec2 {
        vec2(
            self.pos.x + self.width / 2.0,
            self.pos.y + self.height / 2.0,
        )
    }

    pub fn set_center(&mut self, x: f32, y: f32) {
        self.pos.x = x - self.width / 2.0;
        self.pos.y = y - self.height / 2.0;
    }

    pub fn set_center_vec(&mut self, center: Vec2) {
        self.set_center(center.x, center.y);
    }

    pub fn get_top(&self) -> f32 {
        self.pos.y
    }
    pub fn get_bottom(&self) -> f32 {
        self.pos.y + self.height
    }
    pub fn get_right(&self) -> f32 {
        self.pos.x + self.width
    }
    pub fn get_left(&self) -> f32 {
        self.pos.x
    }

    pub fn set_top(&mut self, top: f32) {
        self.pos.y = top;
    }
    pub fn set_bottom(&mut self, bottom: f32) {
        self.pos.y = bottom - self.height;
    }
    pub fn set_right(&mut self, right: f32) {
        self.pos.x = right - self.width;
    }
    pub fn set_left(&mut self, left: f32) {
        self.pos.x = left;
    }

    pub fn touches_rect(&self, rect: &Rect) -> bool {
        self.pos.x < rect.get_right()
            && self.get_right() > rect.pos.x
            && self.pos.y < rect.get_bottom()
            && self.get_bottom() > rect.pos.y
    }

    pub fn touches_line(&self, line: &mut Line) -> bool {
        line.touches_rect(self)
    }

    pub fn touches_point(&self, point: &Vec2) -> bool {
        !(point.x > self.get_right()
            || point.x < self.get_left()
            || point.y > self.get_bottom()
            || point.y < self.get_top())
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, color);
    }
}
