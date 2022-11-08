use std::collections::HashMap;
use std::f32::consts::PI;

use lazy_static::lazy_static;
use macroquad::prelude::{get_frame_time, get_time, is_key_down, is_key_pressed, vec2, KeyCode};
use maplit::hashmap;

use super::main::Player;
use crate::scenes::game::GAME;
use crate::scenes::objects::shapes::rect::Rect;
use crate::util::{deg_to_rad, project, Direction};

lazy_static! {
    static ref ROLL_ANGLES: HashMap<Direction, f32> = hashmap! {
        Direction::WA => deg_to_rad(225.0),
        Direction::WD => deg_to_rad(315.0),
        Direction::SD => deg_to_rad(45.0),
        Direction::SA => deg_to_rad(135.0),
        Direction::W => deg_to_rad(270.0),
        Direction::S => deg_to_rad(90.0),
        Direction::A => deg_to_rad(180.0),
        Direction::D => deg_to_rad(0.0)
    };
}

impl Player {
    pub fn update_movement(&mut self) {
        self.hspd = 0.0;
        self.vspd = 0.0;

        /* -------------------------------- Movement -------------------------------- */
        if is_key_down(KeyCode::W) {
            self.vspd -= 1.0
        }
        if is_key_down(KeyCode::S) {
            self.vspd += 1.0
        }
        if is_key_down(KeyCode::A) {
            self.hspd -= 1.0
        }
        if is_key_down(KeyCode::D) {
            self.hspd += 1.0
        }

        let speed = self.speed * get_frame_time();

        self.hspd *= speed;
        self.vspd *= speed;

        // Fixing diagonal movement
        if self.hspd != 0.0 && self.vspd != 0.0 {
            self.hspd *= 0.707;
            self.vspd *= 0.707;
        }

        self.rolling();

        /* --------------------------- Collision detection -------------------------- */
        self.rect.pos.x += self.hspd;
        for wall in &GAME().walls {
            if self.rect.touches_rect(wall) {
                if self.rect.pos.x > wall.pos.x {
                    self.rect.set_left(wall.get_right());
                } else {
                    self.rect.set_right(wall.get_left());
                }
            }
        }

        self.rect.pos.y += self.vspd;
        for wall in &GAME().walls {
            if self.rect.touches_rect(wall) {
                if self.rect.pos.y > wall.pos.y {
                    self.rect.set_top(wall.get_bottom());
                } else {
                    self.rect.set_bottom(wall.get_top());
                }
            }
        }

        /* ------------------------------ Set direction ----------------------------- */
        let mut temp_direction = "".to_owned();
        if self.vspd > 0.0 {
            temp_direction += "s";
        } else if self.vspd < 0.0 {
            temp_direction += "w";
        }

        if self.hspd > 0.0 {
            temp_direction += "d";
        } else if self.hspd < 0.0 {
            temp_direction += "a";
        }

        self.update_direction();
    }

    fn update_direction(&mut self) {
        self.direction = match ((self.hspd as i32).signum(), (self.vspd as i32).signum()) {
            (1, 0) => Direction::D,
            (-1, 0) => Direction::A,
            (0, 1) => Direction::S,
            (0, -1) => Direction::W,
            (1, 1) => Direction::SD,
            (-1, 1) => Direction::SA,
            (1, -1) => Direction::WD,
            (-1, -1) => Direction::WA,
            (0, 0) => {
                let sheet = self.move_spritesheets.get_mut(&self.direction).unwrap();
                sheet.pause();
                sheet.current_frame = 1;

                // TODO Find out why current_frame of 1 is the first frame, not 0

                self.direction
            }
            _ => panic!(),
        };
    }

    fn rolling(&mut self) {
        self.rolling = get_time() <= self.last_roll + self.roll_duration as f64;
        let on_cooldown =
            get_time() <= self.last_roll + self.roll_duration as f64 + self.roll_cooldown as f64;

        if !on_cooldown && !self.rolling && is_key_pressed(KeyCode::Space) {
            self.roll_angle = *ROLL_ANGLES
                .get(if self.hspd == 0.0 && self.vspd == 0.0 {
                    &Direction::W
                } else {
                    &self.direction
                })
                .unwrap();
            self.last_roll = get_time();
            self.rolling = true;
        }

        if self.rolling {
            let pos = project(
                vec2(0.0, 0.0),
                self.roll_angle,
                self.roll_speed * get_frame_time(),
            );
            self.hspd = pos.x;
            self.vspd = -pos.y;

            self.update_direction();
        }
    }
}
