use macroquad::{prelude::RED, time::get_frame_time};

use crate::{
    game_remove,
    scenes::{game::GAME, object::obj_id, objects::shapes::rect::Rect},
    util::{angle, project},
};

pub(crate) struct Enemy {
    pub rect: Rect,
    speed: f32,
    pub max_health: f32,
    pub health: f32,
    pub id: u32,
}
impl Enemy {
    pub fn new(x: f32, y: f32, max_health: f32) -> Enemy {
        Enemy {
            rect: Rect::new_center(x, y, 30.0, 30.0),
            speed: 100.0,
            max_health,
            health: max_health,
            id: obj_id(),
        }
    }

    pub fn update(&mut self) {
        self.rect.set_center_vec(project(
            self.rect.get_center(),
            angle(self.rect.get_center(), GAME().player.rect.get_center()),
            self.speed * get_frame_time(),
        ));
    }

    pub fn draw(&mut self) {
        self.rect.draw(RED);
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }

    pub fn hit(&mut self, damage: f32) {
        self.health -= damage;
        if self.health < 0.0 {
            game_remove!(GAME().enemies, self.id);
        }
    }
}
