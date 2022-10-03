use macroquad::{
    prelude::{Vec2, YELLOW},
    rand::gen_range,
    time::{get_frame_time, get_time},
};

use crate::{
    game_remove,
    scenes::{
        game::{self, GAME},
        object::{obj_id, IDObject},
    },
    util::{deg_to_rad, project},
};

use super::shapes::rect::Rect;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BulletConfig {
    pub speed: f32,
    pub max_lifespan: f32,
    /// Bullet spread in degrees
    pub spread: f32,
    pub bullet_size: f32,
    /// Wether the bullet can travel through enemies (0 if not)
    pub pierce: u8,
    pub damage: f32,
    /// If false, will damage player
    pub friendly: bool,
}

pub(crate) struct Bullet {
    angle: f32,
    rect: Rect,
    created: f64,
    config: BulletConfig,
    id: u32,
}
impl Bullet {
    pub fn new(angle: f32, pos: Vec2, config: BulletConfig) -> Bullet {
        let spread = if config.spread != 0.0 {
            deg_to_rad(gen_range(-config.spread, config.spread))
        } else {
            0.0
        };

        return Bullet {
            angle: angle + spread,
            rect: Rect::new_center_vec(pos, config.bullet_size, config.bullet_size),
            config,
            created: get_time(),
            id: obj_id(),
        };
    }
}
impl IDObject for Bullet {
    fn update(&mut self) {
        if get_time() > self.created + self.config.max_lifespan as f64 {
            game_remove!(GAME().objects, self.id);
        }

        self.rect.set_center_vec(project(
            self.rect.get_center(),
            self.angle,
            self.config.speed * get_frame_time(),
        ));

        if self.config.friendly {
            for enemy in &mut GAME().enemies {
                if self.rect.touches(&enemy.rect) {
                    enemy.hit(self.config.damage);
                }
            }
        } else {
            if self.rect.touches(&GAME().player.rect) {
                GAME().player.hit(self.config.damage);
            }
        }
    }

    fn draw(&mut self) {
        self.rect.draw(YELLOW);
    }

    fn get_id(&self) -> u32 {
        return self.id;
    }
}
