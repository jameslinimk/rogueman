use macroquad::{
    prelude::{vec2, Color, Vec2},
    shapes::draw_line,
};

use crate::util::distance;

use super::rect::Rect;

pub fn polygons_intersect(polygons: &[Vec<Vec2>; 2]) -> bool {
    for polygon in polygons {
        for (i1, p1) in polygon.iter().enumerate() {
            let i2 = (i1 + 1) % polygon.len();
            let p2 = polygon[i2];

            let normal = vec2(p2.y - p1.y, p1.x - p2.x);

            let mut min_a: Option<f32> = None;
            let mut max_a: Option<f32> = None;
            for p in &polygons[0] {
                let projected = normal.x * p.x + normal.y * p.y;
                if min_a.is_none() || projected < min_a.unwrap() {
                    min_a = Option::from(projected);
                }
                if min_a.is_none() || projected > max_a.unwrap() {
                    max_a = Option::from(projected);
                }
            }

            let mut min_b: Option<f32> = None;
            let mut max_b: Option<f32> = None;
            for p in &polygons[1] {
                let projected = normal.x * p.x + normal.y * p.y;
                if min_b.is_none() || projected < min_b.unwrap() {
                    min_b = Option::from(projected);
                }
                if max_b.is_none() || projected > max_b.unwrap() {
                    max_b = Option::from(projected);
                }
            }

            if max_a.unwrap() < min_b.unwrap() || max_b.unwrap() > min_a.unwrap() {
                return false;
            }
        }
    }

    return true;
}

pub struct Line {
    p1: Vec2,
    p2: Vec2,
    thickness: f32,
    pub points: Vec<Vec2>,
}
impl Line {
    pub fn new(p1: Vec2, p2: Vec2, thickness: f32) -> Line {
        let width = p2.x - p1.x;
        let height = p2.y - p1.y;
        let length = distance(p1, p2);
        let xs = (thickness * height / length) / 2.0;
        let ys = (thickness * width / length) / 2.0;

        Line {
            p1,
            p2,
            thickness,
            points: vec![
                vec2(p1.x - xs, p1.y - ys),
                vec2(p1.x + xs, p1.y + ys),
                vec2(p2.x + xs, p2.y - ys),
                vec2(p2.x - xs, p2.y - ys),
            ],
        }
    }

    pub fn draw(&self, color: Color) {
        draw_line(
            self.p1.x,
            self.p1.y,
            self.p2.x,
            self.p2.y,
            self.thickness,
            color,
        );
    }

    pub fn touches_line(&self, rect: &Rect) -> bool {
        polygons_intersect(&[
            self.points.to_vec(),
            vec![
                vec2(rect.pos.x, rect.pos.y),
                vec2(rect.pos.x + rect.width, rect.pos.y),
                vec2(rect.pos.x + rect.width, rect.pos.y + rect.height),
                vec2(rect.pos.x, rect.pos.y + rect.height),
            ],
        ])
    }
}
