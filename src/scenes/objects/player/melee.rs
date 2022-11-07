use super::player::Player;
use crate::scenes::objects::items::melee::Melee;
use crate::scenes::objects::shapes::line::Line;
use crate::unwrap_or_return;
use crate::util::angle;
use crate::util::project;
use crate::util::rel_mouse_pos;
use crate::util::NUMBER_KEYS;
use crate::GAME;
use macroquad::prelude::{get_time, is_key_pressed, is_mouse_button_pressed, MouseButton, YELLOW};

impl Player {
    pub fn update_melee(&mut self) {
        let melee = unwrap_or_return!(self.get_melee());

        for i in 1..=3 {
            let key = NUMBER_KEYS[i - 1];
            if is_key_pressed(key) && i < self.melees.len() {
                self.selected_melee = i;
            }
        }

        let (mut swinging, on_cooldown) = self.melee_info(melee);

        /* ----------------------------- Start of swing ----------------------------- */
        if !on_cooldown && is_mouse_button_pressed(MouseButton::Left) {
            self.last_melee_angle = Option::from(angle(self.rect.get_center(), rel_mouse_pos()));
            self.last_melee_line = Option::from(Line::new(
                self.rect.get_center(),
                project(
                    self.rect.get_center(),
                    self.last_melee_angle.unwrap(),
                    melee.range,
                ),
                melee.range_width,
            ));

            self.last_melee = get_time();
            swinging = true;
        }

        /* -------------------------------- Swinging -------------------------------- */
        if swinging {
            // Calculating line
            let line = self.last_melee_line.as_mut().unwrap();
            line.p1 = self.rect.get_center();
            line.p2 = project(
                self.rect.get_center(),
                self.last_melee_angle.unwrap(),
                melee.range,
            );

            // Hitting enemies
            for enemy in &mut GAME().enemies {
                if line.touches_rect(&enemy.rect) {
                    enemy.hit(melee.damage);
                }
            }
        }
    }

    pub fn draw_melee(&mut self) {
        let melee = unwrap_or_return!(self.get_melee());

        let (swinging, _) = self.melee_info(melee);
        if swinging {
            self.last_melee_line.as_ref().unwrap().draw(YELLOW);
        }
    }

    /// Returns `(swinging, on_cooldown)`
    fn melee_info(&mut self, melee: Melee) -> (bool, bool) {
        if self.last_melee == 0.0
            || get_time() > self.last_melee + melee.delay as f64 + melee.swing_duration as f64
        {
            (false, false)
        } else {
            (
                get_time() <= self.last_melee + melee.swing_duration as f64,
                true,
            )
        }
    }

    pub fn get_melee(&self) -> Option<Melee> {
        if self.melees.is_empty() || self.selected_melee >= self.melees.len() {
            return None;
        }
        Option::from(self.melees[self.selected_melee])
    }
}
