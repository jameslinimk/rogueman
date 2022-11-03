use super::player::Player;
use crate::scenes::game::GAME;
use crate::scenes::objects::bullet::Bullet;
use crate::scenes::objects::items::guns::Gun;
use crate::scenes::objects::objects::Objects;
use crate::util::angle;
use crate::util::rel_mouse_pos;
use crate::KeyCode;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::is_mouse_button_down;
use macroquad::prelude::is_mouse_button_pressed;
use macroquad::prelude::MouseButton;
use macroquad::time::get_time;

impl Player {
    pub fn update_shoot(&mut self) {
        let gun = match self.get_gun() {
            Some(gun) => gun,
            None => return,
        };

        /* ---------------------------- Switching weapons --------------------------- */
        if is_key_pressed(KeyCode::G) {
            self.selected_gun += 1;
            if self.selected_gun >= self.guns.len() {
                self.selected_gun = 0;
            }
        }

        /* -------------------------------- Shooting -------------------------------- */
        if if gun.holdable {
            is_mouse_button_down(MouseButton::Right)
        } else {
            is_mouse_button_pressed(MouseButton::Right)
        } {
            self.shoot();
        }
    }

    fn shoot(&mut self) {
        let gun = match self.get_gun() {
            Some(gun) => gun,
            None => return,
        };

        if self.last_shot == 0.0 || get_time() > self.last_shot + gun.fire_delay as f64 {
            GAME().camera.set_shake(gun.shake);

            let angle = angle(self.rect.get_center(), rel_mouse_pos());
            GAME().objects.push(Objects::from(Bullet::new(
                angle,
                self.rect.get_center(),
                gun.bullet_config,
            )));
            self.last_shot = get_time();
        }
    }

    pub fn get_gun(&self) -> Option<Gun> {
        if self.guns.len() == 0 || self.selected_gun >= self.guns.len() {
            return None;
        }
        return Option::from(self.guns[self.selected_gun]);
    }
}
