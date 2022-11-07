use std::collections::HashMap;
use std::f32::consts::PI;

use lazy_static::lazy_static;
use macroquad::prelude::{get_frame_time, get_time, is_key_down, is_key_pressed, vec2, KeyCode};
use maplit::hashmap;

use super::main::Player;
use crate::scenes::game::GAME;
use crate::util::{deg_to_rad, project};

lazy_static! {
    static ref ROLL_ANGLES: HashMap<String, f32> = hashmap! {
        "wa".to_owned() => deg_to_rad(225.0),
        "wd".to_owned() => deg_to_rad(315.0),
        "sd".to_owned() => deg_to_rad(45.0),
        "sa".to_owned() => deg_to_rad(135.0),
        "w".to_owned() => deg_to_rad(270.0),
        "s".to_owned() => deg_to_rad(90.0),
        "a".to_owned() => deg_to_rad(180.0),
        "d".to_owned() => deg_to_rad(0.0)
    };
}

impl Player {
    pub fn update_movement(&mut self) {
        /* -------------------------------- Movement -------------------------------- */
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

        let speed = self.speed * get_frame_time();

        hspd *= speed;
        vspd *= speed;

        // Fixing diagonal movement
        if hspd != 0.0 && vspd != 0.0 {
            hspd *= 0.707;
            vspd *= 0.707;
        }

        self.rolling(&mut vspd, &mut hspd);

        /* --------------------------- Collision detection -------------------------- */
        self.rect.pos.x += hspd;
        for wall in &GAME().walls {
            if self.rect.touches_rect(wall) {
                if self.rect.pos.x > wall.pos.x {
                    self.rect.set_left(wall.get_right());
                } else {
                    self.rect.set_right(wall.get_left());
                }
            }
        }

        self.rect.pos.y += vspd;
        for wall in &GAME().walls {
            if self.rect.touches_rect(wall) {
                if self.rect.pos.y > wall.pos.y {
                    self.rect.set_top(wall.get_bottom());
                } else {
                    self.rect.set_bottom(wall.get_top());
                }
            }
        }
    }

    fn rolling(&mut self, vspd: &mut f32, hspd: &mut f32) {
        let mut rolling = get_time() <= self.last_roll + self.roll_duration as f64;
        let on_cooldown =
            get_time() <= self.last_roll + self.roll_duration as f64 + self.roll_cooldown as f64;

        if !on_cooldown && !rolling && is_key_pressed(KeyCode::Space) {
            let mut dash_angle = "".to_owned();
            if *vspd > 0.0 {
                dash_angle += "s";
            } else if *vspd < 0.0 {
                dash_angle += "w";
            }

            if *hspd > 0.0 {
                dash_angle += "d";
            } else if *hspd < 0.0 {
                dash_angle += "a";
            }

            if dash_angle.is_empty() {
                dash_angle += "w";
            }

            self.roll_angle = *ROLL_ANGLES.get(&dash_angle).unwrap();
            self.last_roll = get_time();
            rolling = true;
        }

        if rolling {
            let pos = project(
                vec2(0.0, 0.0),
                self.roll_angle,
                self.roll_speed * get_frame_time(),
            );
            *hspd = pos.x;
            *vspd = -pos.y;
        }
    }
}
