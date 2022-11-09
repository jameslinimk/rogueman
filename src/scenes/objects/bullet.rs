use macroquad::prelude::rand::gen_range;
use macroquad::prelude::{get_frame_time, get_time, Vec2, YELLOW};

use super::shapes::rect::Rect;
use crate::scenes::game::GAME;
use crate::scenes::object::{obj_id, IDObject};
use crate::util::{deg_to_rad, project};

#[derive(Debug, Clone, Copy)]
pub struct BulletConfig {
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

pub struct Bullet {
    angle: f32,
    rect: Rect,
    created: f64,
    traveled_through: u8,
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

        Bullet {
            traveled_through: 0,
            angle: angle + spread,
            rect: Rect::new_center_vec(pos, config.bullet_size, config.bullet_size),
            config,
            created: get_time(),
            id: obj_id(),
        }
    }
}
impl Bullet {
    fn update_collision(&mut self) {
        for wall in &mut GAME().walls {
            if self.rect.touches_rect(wall) {
                GAME().remove_object(self.id);
                return;
            }
        }

        if self.config.friendly {
            for enemy in &mut GAME().enemies {
                if self.rect.touches_rect(&enemy.rect) {
                    let success = enemy.hit(self.config.damage);

                    if success {
                        self.traveled_through += 1;
                        if self.traveled_through > self.config.pierce {
                            GAME().remove_object(self.id);
                            return;
                        }
                    }
                }
            }
        } else if self.rect.touches_rect(&GAME().player.rect) {
            let success = GAME().player.hit(self.config.damage);

            if success {
                self.traveled_through += 1;
                if self.traveled_through > self.config.pierce {
                    GAME().remove_object(self.id);
                }
            }
        }
    }
}
impl IDObject for Bullet {
    fn update(&mut self) {
        if get_time() > self.created + self.config.max_lifespan as f64 {
            GAME().remove_object(self.id);
        }

        self.rect.set_center_vec(project(
            self.rect.get_center(),
            self.angle,
            self.config.speed * get_frame_time(),
        ));

        self.update_collision();
    }

    fn draw(&mut self) {
        self.rect.draw(YELLOW);
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
