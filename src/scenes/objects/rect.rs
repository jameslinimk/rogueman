use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::shapes::draw_rectangle;

pub (crate) struct Rect {
    pub(crate) pos: Vec2,
    pub(crate) width: f32,
    pub(crate) height: f32
}
impl Rect {
    /// Create a new rect from top-left point
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            pos: Vec2::new(x, y),
            width,
            height
        }
    }

    /// Creates new rect from a center point
    pub fn new_center(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            pos: Vec2::new(x - width / 2.0, y - width / 2.0),
            width,
            height
        }
    }

    pub fn get_center(&self) -> Vec2 {
        return Vec2::new(self.pos.x - self.width / 2.0, self.pos.y - self.height / 2.0);
    }

    pub fn set_center(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, color);
    }

    pub fn touches(&self, rect: Rect) -> bool {
        self.pos.x < rect.pos.x ||
            self.pos.y < rect.pos.y ||
            self.pos.x - self.width > rect.pos.x - rect.width ||
            self.pos.y - self.height > self.pos.y - rect.height
    }
}
