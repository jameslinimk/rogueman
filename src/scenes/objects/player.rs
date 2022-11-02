use crate::scenes::objects::shapes::line::Line;
use crate::scenes::objects::shapes::rect::Rect;
use crate::util::{
    angle, multiline_text, project, rel_mouse_pos, rx_smooth, ry_smooth, DAMAGE_COOLDOWN,
    NUMBER_KEYS,
};
use crate::{KeyCode, GAME};
use macroquad::color::WHITE;
use macroquad::input::is_key_down;
use macroquad::prelude::{
    is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, Color, MouseButton, YELLOW,
};
use macroquad::shapes::draw_rectangle;

use macroquad::texture::draw_texture;
use macroquad::time::{get_frame_time, get_time};
use macroquad::window::screen_height;

use super::assets::get_image;
use super::bullet::Bullet;

use super::items::guns::{Gun, GUNS};
use super::items::melee::{Melee, MELEES};
use super::objects::Objects;

#[derive(Debug)]
pub struct Player {
    pub rect: Rect,
    speed: f32,
    max_health: f32,
    health: f32,
    last_damage: f64,

    guns: Vec<Gun>,
    selected_gun: usize,
    last_shot: f64,

    melees: Vec<Melee>,
    selected_melee: usize,
    last_melee: f64,
    last_melee_angle: Option<f32>,
    last_melee_line: Option<Line>,
}
impl Player {
    pub fn new() -> Player {
        Player {
            rect: Rect::new_center(-100.0, -100.0, 30.0, 30.0),
            speed: 500.0,
            max_health: 100.0,
            health: 100.0,
            last_damage: 0.0,

            guns: GUNS.to_vec(),
            selected_gun: 0,
            last_shot: 0.0,

            melees: MELEES.to_vec(),
            selected_melee: 0,
            last_melee: 0.0,
            last_melee_angle: None,
            last_melee_line: None,
        }
    }

    pub fn update(&mut self) {
        self.update_movement();
        self.update_shoot();
        self.update_melee();

        GAME().camera.target = self.rect.get_center();
    }

    fn update_movement(&mut self) {
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

        let dt = get_frame_time();
        let speed = self.speed * dt;

        hspd *= speed;
        vspd *= speed;

        // Fixing diagonal movement
        if hspd != 0.0 && vspd != 0.0 {
            hspd *= 0.707;
            vspd *= 0.707;
        }

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

    fn update_shoot(&mut self) {
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

    fn update_melee(&mut self) {
        let melee = match self.get_melee() {
            Some(melee) => melee,
            None => return,
        };

        for (i, key) in NUMBER_KEYS.iter().enumerate() {
            if is_key_pressed(*key) && i < self.melees.len() {
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

    fn draw_melee(&mut self) {
        let melee = match self.get_melee() {
            Some(melee) => melee,
            None => return,
        };

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

    fn get_gun(&self) -> Option<Gun> {
        if self.guns.len() == 0 || self.selected_gun >= self.guns.len() {
            return None;
        }
        return Option::from(self.guns[self.selected_gun]);
    }

    fn get_melee(&self) -> Option<Melee> {
        if self.melees.len() == 0 || self.selected_melee >= self.melees.len() {
            return None;
        }
        return Option::from(self.melees[self.selected_melee]);
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

    fn draw_ui(&self) {
        /* ------------------------------- Debug Menu ------------------------------- */
        let gun = self.get_gun();
        let melee = self.get_melee();
        multiline_text(
            &format!(
                "X,Y: {}, {}\nGun: {}\nMelee: {}",
                self.rect.get_center().x.round(),
                self.rect.get_center().y.round(),
                if gun.is_none() {
                    "None"
                } else {
                    &gun.unwrap().name
                },
                if melee.is_none() {
                    "None"
                } else {
                    &melee.unwrap().name
                },
            ),
            rx_smooth(0.0),
            ry_smooth(27.0),
            50,
            WHITE,
        );

        /* -------------------------------- Gun info -------------------------------- */
        if gun.is_some() {
            let x = rx_smooth(10.0);
            let y = ry_smooth(screen_height() - 74.0);

            /* --------------------------------- Border --------------------------------- */
            let border_texture = get_image("./assets/guns/border.png").unwrap();
            draw_texture(border_texture, x, y, WHITE);

            /* -------------------------------- Gun image ------------------------------- */
            let texture = get_image(gun.unwrap().image_file).unwrap();
            draw_texture(texture, x, y, WHITE);

            /* ---------------------------- Shooting cooldown --------------------------- */
            let fire_delay = gun.unwrap().fire_delay as f64;
            let ratio =
                ((get_time() - self.last_shot).clamp(0.0, fire_delay) - fire_delay) / fire_delay;
            draw_rectangle(
                x,
                y,
                64.0,
                64.0 * ratio as f32 * -1.0,
                Color::from_rgba(0, 0, 0, 120),
            );
        }
    }

    pub fn draw(&mut self) {
        self.rect.draw(WHITE);

        // let mouse_pos = rel_mouse_pos();

        // // Extend line by length
        // let length = screen_height() + screen_width();
        // let alpha =
        //     (mouse_pos.y - self.rect.get_center().y).atan2(mouse_pos.x - self.rect.get_center().x);

        // draw_line(
        //     self.rect.get_center().x,
        //     self.rect.get_center().y,
        //     mouse_pos.x + length * alpha.cos(),
        //     mouse_pos.y + length * alpha.sin(),
        //     2.0,
        //     BLUE,
        // );

        self.draw_melee();
        self.draw_ui();
    }

    pub fn hit(&mut self, damage: f32) -> bool {
        if get_time() <= self.last_damage + DAMAGE_COOLDOWN {
            return false;
        }

        self.last_damage = get_time();
        self.health -= damage;
        if self.health < 0.0 {
            println!("Player died");
        }
        return true;
    }
}
