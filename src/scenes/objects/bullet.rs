use macroquad::{
    prelude::{Vec2, YELLOW},
    time::{get_frame_time, get_time},
};

use crate::{scenes::object::Object, util::project};

use super::shapes::rect::Rect;

pub(crate) struct Bullet {
    angle: f32,
    rect: Rect,
    speed: f32,
    max_lifespan: f32,
    created: f64,
}
impl Bullet {
    pub fn new(angle: f32, pos: Vec2) -> Bullet {
        Bullet {
            angle,
            rect: Rect::new_center_vec(pos, 20.0, 20.0),
            speed: 150.0,
            max_lifespan: 2.0,
            created: get_time(),
        }
    }
}
impl Object for Bullet {
    fn update(&mut self) {
        if get_time() > self.created + self.max_lifespan as f64 {
            println!("delete");
        }

        self.rect.set_center_vec(project(
            self.rect.get_center(),
            self.angle,
            self.speed * get_frame_time(),
        ));
    }

    fn draw(&mut self) {
        self.rect.draw(YELLOW);
    }
}
