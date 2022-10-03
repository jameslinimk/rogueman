use crate::camera::CAMERA;
use crate::scenes::objects::shapes::rect::Rect;
use crate::util::{angle, rel_mouse_pos};
use crate::{KeyCode, GAME};
use macroquad::color::{BLUE, WHITE};
use macroquad::input::is_key_down;
use macroquad::prelude::{is_mouse_button_pressed, MouseButton, is_mouse_button_down};
use macroquad::shapes::draw_line;
use macroquad::time::{get_frame_time, get_time};
use macroquad::window::{screen_height, screen_width};

use super::bullet::{Bullet, BulletConfig};
use super::guns::{pistol, Gun};

#[derive(Debug)]
pub(crate) struct Player {
    pub rect: Rect,
    speed: f32,
    selected_gun: Option<Gun>,
    last_shot: f64,
    max_health: f32,
    health: f32
}
impl Player {
    pub fn new() -> Player {
        Player {
            rect: Rect::new_center(0.0, 0.0, 30.0, 30.0),
            speed: 500.0,
            selected_gun: Option::from(pistol()),
            last_shot: 0.0,
            max_health: 100.0,
            health: 100.0
        }
    }
}
impl Player {
    #[rustfmt::skip]
    pub fn update(&mut self) {
        let mut hspd = 0.0;
        let mut vspd = 0.0;

        if is_key_down(KeyCode::W) { vspd -= 1.0 }
        if is_key_down(KeyCode::S) { vspd += 1.0 }
        if is_key_down(KeyCode::A) { hspd -= 1.0 }
        if is_key_down(KeyCode::D) { hspd += 1.0 }

        // Band-aid patch for diagonal movement
        let dia = if hspd != 0.0 && vspd != 0.0 { 0.707 } else { 1.0 };

        let ft = get_frame_time();
        hspd *= self.speed * ft * dia;
        vspd *= self.speed * ft * dia;

        // Collision detection
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

        if self.selected_gun.is_some() && get_time() > self.last_shot + self.selected_gun.as_ref().unwrap().fire_delay as f64 {
            if self.selected_gun.as_ref().unwrap().holdable && is_mouse_button_down(MouseButton::Left) {
                self.shoot();
            } else if is_mouse_button_pressed(MouseButton::Left) {
                self.shoot();
            }
        }
        
        CAMERA().target = self.rect.get_center();
    }

    fn shoot(&mut self) {
        if self.selected_gun.is_none() { return }
        let angle = angle(self.rect.get_center(), rel_mouse_pos());
        GAME().objects.push(Box::new(Bullet::new(angle, self.rect.get_center(), self.selected_gun.as_ref().unwrap().bullet_config)));
        self.last_shot = get_time();
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
    }

    pub fn hit(&mut self, damage: f32) {
        self.health -= damage;
        if self.health < 0.0 {
            println!("Player died");
        }
    }
}
