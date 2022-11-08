use std::collections::HashMap;

use derive_new::new;
use macroquad::prelude::{draw_rectangle, draw_texture, get_time, screen_height, Color, WHITE};
use maplit::hashmap;

use crate::scenes::objects::assets::{get_image, load_image};
use crate::scenes::objects::items::guns::{Gun, GUNS};
use crate::scenes::objects::items::melee::{Melee, MELEES};
use crate::scenes::objects::shapes::line::Line;
use crate::scenes::objects::shapes::rect::Rect;
use crate::spritesheet::SpriteSheet;
use crate::util::{multiline_text, rx_smooth, ry_smooth, DAMAGE_COOLDOWN, DIRECTIONS};
use crate::{repeat_function, GAME};

#[derive(Debug, new)]
pub struct Player {
    #[new(value = "Rect::new_center(-100.0, -100.0, 30.0, 30.0)")]
    pub rect: Rect,
    #[new(value = "500.0")]
    pub speed: f32,
    #[new(value = "100.0")]
    pub max_health: f32,
    #[new(value = "100.0")]
    pub health: f32,
    #[new(value = "f64::MIN")]
    pub last_damage: f64,
    #[new(value = "false")]
    pub invulnerable: bool,

    #[new(value = "GUNS.to_vec()")]
    pub guns: Vec<Gun>,
    #[new(value = "0")]
    pub selected_gun: usize,
    #[new(value = "f64::MIN")]
    pub last_shot: f64,

    #[new(value = "MELEES.to_vec()")]
    pub melees: Vec<Melee>,
    #[new(value = "0")]
    pub selected_melee: usize,
    #[new(value = "f64::MIN")]
    pub last_melee: f64,
    #[new(value = "None")]
    pub last_melee_angle: Option<f32>,
    #[new(value = "None")]
    pub last_melee_line: Option<Line>,

    #[new(value = "f64::MIN")]
    pub last_roll: f64,
    #[new(value = "0.1")]
    pub roll_duration: f32,
    #[new(value = "0.5")]
    pub roll_cooldown: f32,
    #[new(value = "0.0")]
    pub roll_angle: f32,
    #[new(value = "1600.0")]
    pub roll_speed: f32,

    #[new(value = "hashmap! {
        \"w\" => SpriteSheet::new(get_image(\"./assets/player/movement/move_w.png\"), 4, 0.1),
        \"a\" => SpriteSheet::new(get_image(\"./assets/player/movement/move_a.png\"), 4, 0.1),
        \"s\" => SpriteSheet::new(get_image(\"./assets/player/movement/move_s.png\"), 4, 0.1),
        \"d\" => SpriteSheet::new(get_image(\"./assets/player/movement/move_d.png\"), 4, 0.1),
        \"wa\" => SpriteSheet::new(get_image(\"./assets/player/movement/move_wa.png\"), 4, 0.1),
        \"wd\" => SpriteSheet::new(get_image(\"./assets/player/movement/move_wd.png\"), 4, 0.1),
        \"sa\" => SpriteSheet::new(get_image(\"./assets/player/movement/move_sa.png\"), 4, 0.1),
        \"sd\" => SpriteSheet::new(get_image(\"./assets/player/movement/move_sd.png\"), 4, 0.1),
    }")]
    pub move_spritesheets: HashMap<&'static str, SpriteSheet>,
}
impl Player {
    pub async fn init() {
        // TODO
        // for dir in &DIRECTIONS {
        //     load_image(&format!("./assets/player/movement/move_{}.png", dir)).await;
        // }
    }

    pub fn update(&mut self) {
        self.update_movement();
        self.update_shoot();
        self.update_melee();
        for dir in &DIRECTIONS {
            self.move_spritesheets.get_mut(dir).unwrap().update();
        }

        GAME().camera.target = self.rect.get_center();
    }

    pub fn draw(&mut self) {
        self.rect.draw(WHITE);
        let center = self.rect.get_center();

        for dir in &DIRECTIONS {
            self.move_spritesheets
                .get(dir)
                .unwrap()
                .draw(center.x - 32.0, center.y - 32.0, 64.0);
        }

        self.draw_melee();
        self.draw_ui();
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
                    gun.unwrap().name
                },
                if melee.is_none() {
                    "None"
                } else {
                    melee.unwrap().name
                },
            ),
            rx_smooth(0.0),
            ry_smooth(27.0),
            50,
            WHITE,
        );

        /* -------------------------------- Gun info -------------------------------- */
        if let Some(g) = gun {
            let x = rx_smooth(10.0);
            let y = ry_smooth(screen_height() - 74.0);

            /* --------------------------------- Border --------------------------------- */
            let border_texture = get_image("./assets/guns/border.png");
            draw_texture(border_texture, x, y, WHITE);

            /* -------------------------------- Gun image ------------------------------- */
            let texture = get_image(g.image_file);
            draw_texture(texture, x, y, WHITE);

            /* ---------------------------- Shooting cooldown --------------------------- */
            let fire_delay = g.fire_delay as f64;
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

    pub fn hit(&mut self, damage: f32) -> bool {
        if self.invulnerable || get_time() <= self.last_damage + DAMAGE_COOLDOWN {
            return false;
        }

        self.last_damage = get_time();
        self.health -= damage;
        if self.health < 0.0 {
            println!("Player died");
        }

        true
    }
}
