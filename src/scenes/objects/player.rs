use crate::camera::{ShakeConfig, CAMERA};
use crate::scenes::objects::shapes::rect::Rect;
use crate::util::{
    angle, multiline_text, rel_mouse_pos, rx, rx_smooth, ry, ry_smooth, NUMBER_KEYS,
};
use crate::{KeyCode, GAME};
use macroquad::color::{BLUE, WHITE};
use macroquad::input::is_key_down;
use macroquad::prelude::{
    is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, MouseButton,
};
use macroquad::shapes::draw_line;
use macroquad::text::draw_text;
use macroquad::texture::draw_texture;
use macroquad::time::{get_frame_time, get_time};
use macroquad::window::{screen_height, screen_width};

use super::assets::get_image;
use super::bullet::{Bullet, BulletConfig};
use super::guns::{Gun, GUNS};

#[derive(Debug)]
pub(crate) struct Player {
    pub rect: Rect,
    speed: f32,
    last_shot: f64,
    max_health: f32,
    health: f32,
    guns: Vec<Gun>,
    selected_gun: usize,
}
impl Player {
    pub fn new() -> Player {
        Player {
            rect: Rect::new_center(0.0, 0.0, 30.0, 30.0),
            speed: 500.0,
            last_shot: 0.0,
            max_health: 100.0,
            health: 100.0,
            guns: GUNS.to_vec(),
            selected_gun: 0,
        }
    }

    pub fn update(&mut self) {
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

        // Band-aid patch for diagonal movement
        let dia = if hspd != 0.0 && vspd != 0.0 {
            0.707
        } else {
            1.0
        };

        let ft = get_frame_time();
        hspd *= self.speed * ft * dia;
        vspd *= self.speed * ft * dia;

        /* --------------------------- Collision detection -------------------------- */
        for wall in &GAME().walls {
            self.rect.pos.x += hspd;
            if self.rect.touches(wall) {
                if self.rect.pos.x > wall.pos.x {
                    self.rect.set_left(wall.get_right());
                } else {
                    self.rect.set_right(wall.get_left());
                }
            }

            self.rect.pos.y += vspd;
            if self.rect.touches(wall) {
                if self.rect.pos.y > wall.pos.y {
                    self.rect.set_top(wall.get_bottom());
                } else {
                    self.rect.set_bottom(wall.get_top());
                }
            }
        }

        /* -------------------------------- Shooting -------------------------------- */
        match self.get_gun() {
            Some(gun) => {
                if gun.holdable && is_mouse_button_down(MouseButton::Left) {
                    self.shoot();
                } else if is_mouse_button_pressed(MouseButton::Left) {
                    self.shoot();
                }
            }
            None => {}
        };

        /* ---------------------------- Switching weapons --------------------------- */
        for (i, key) in NUMBER_KEYS.iter().enumerate() {
            if is_key_pressed(*key) && i < self.guns.len() {
                self.selected_gun = i;
            }
        }

        /* ---------------------------------- Misc ---------------------------------- */
        CAMERA().target = self.rect.get_center();
    }

    fn get_gun(&self) -> Option<Gun> {
        if self.selected_gun >= self.guns.len() {
            return None;
        }
        return Option::from(self.guns[self.selected_gun]);
    }

    fn shoot(&mut self) {
        match self.get_gun() {
            Some(gun) => {
                if self.last_shot == 0.0 || get_time() > self.last_shot + gun.fire_delay as f64 {
                    CAMERA().set_shake(gun.shake);

                    let angle = angle(self.rect.get_center(), rel_mouse_pos());
                    GAME().objects.push(Box::new(Bullet::new(
                        angle,
                        self.rect.get_center(),
                        gun.bullet_config,
                    )));
                    self.last_shot = get_time();
                }
            }
            None => {}
        }
    }

    fn draw_ui(&self) {
        let gun = self.get_gun();
        multiline_text(
            &format!(
                "X,Y: {}, {}\nGun: {}",
                self.rect.get_center().x.round(),
                self.rect.get_center().y.round(),
                if gun.is_none() {
                    "None"
                } else {
                    &gun.unwrap().name
                }
            ),
            rx_smooth(0.0),
            ry_smooth(27.0),
            50,
            WHITE,
        );

        async {
            if gun.is_some() {
                println!("draw");
                let gun_image = get_image(gun.unwrap().image_file).await;
                draw_texture(gun_image, rx_smooth(100.0), ry_smooth(100.0), WHITE);
            }
        }
    }

    pub fn draw(&mut self) {
        self.rect.draw(WHITE);

        let mouse_pos = rel_mouse_pos();

        // Extend line by length
        let length = screen_height() + screen_width();
        let alpha =
            (mouse_pos.y - self.rect.get_center().y).atan2(mouse_pos.x - self.rect.get_center().x);

        draw_line(
            self.rect.get_center().x,
            self.rect.get_center().y,
            mouse_pos.x + length * alpha.cos(),
            mouse_pos.y + length * alpha.sin(),
            2.0,
            BLUE,
        );

        self.draw_ui();
    }

    pub fn hit(&mut self, damage: f32) {
        self.health -= damage;
        if self.health < 0.0 {
            println!("Player died");
        }
    }
}
