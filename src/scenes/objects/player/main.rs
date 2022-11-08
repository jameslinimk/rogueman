use std::collections::HashMap;

use derive_new::new;
use macroquad::prelude::{draw_rectangle, draw_texture, get_time, screen_height, Color, WHITE};
use maplit::hashmap;

use crate::scenes::objects::assets::{get_image, get_image_owned, load_image, load_image_owned};
use crate::scenes::objects::items::guns::{Gun, GUNS};
use crate::scenes::objects::items::melee::{Melee, MELEES};
use crate::scenes::objects::shapes::line::Line;
use crate::scenes::objects::shapes::rect::Rect;
use crate::spritesheet::SpriteSheet;
use crate::util::{
    multiline_text, rx_smooth, ry_smooth, Direction, DAMAGE_COOLDOWN, DIRECTIONS, SQUARE_SIZE,
};
use crate::{repeat_function, GAME};

#[derive(Debug, new)]
pub struct Player {
    #[new(value = "Rect::new_center(-100.0, -100.0, SQUARE_SIZE, SQUARE_SIZE)")]
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

    #[new(value = "0.0")]
    pub hspd: f32,
    #[new(value = "0.0")]
    pub vspd: f32,
    #[new(value = "Direction::S")]
    pub direction: Direction,

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

    #[new(value = "false")]
    pub rolling: bool,
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

    #[new(value = "{
        let mut temp_map = hashmap! {};
        for dir in &DIRECTIONS {
            temp_map.insert(
                dir.0,
                SpriteSheet::new(
                    get_image_owned(format!(\"./assets/player/movement/move_{}.png\", dir.1)),
                    4,
                    0.1
                ),
            );
        }
        temp_map
    }")]
    pub move_spritesheets: HashMap<Direction, SpriteSheet>,
    #[new(value = "{
        let mut temp_map = hashmap! {};

        // Sync this with line 60
        let roll_duration = 0.1;
        let width = 4;

        for dir in &DIRECTIONS {
            temp_map.insert(
                dir.0,
                SpriteSheet::new(
                    get_image_owned(format!(\"./assets/player/movement/roll_{}.png\", dir.1)),
                    width,
                    roll_duration / width as f32
                ),
            );
        }
        temp_map
    }")]
    pub roll_spritesheets: HashMap<Direction, SpriteSheet>,
}
impl Player {
    pub async fn init() {
        for (_, dir) in &DIRECTIONS {
            load_image_owned(format!("./assets/player/movement/move_{}.png", dir)).await;
            load_image_owned(format!("./assets/player/movement/roll_{}.png", dir)).await;
        }
    }

    pub fn update(&mut self) {
        self.update_movement();
        self.update_shoot();
        self.update_melee();

        /* ----------------------------- Drawing sprite ----------------------------- */
        self.move_spritesheets
            .get_mut(&self.direction)
            .unwrap()
            .update();
        if self.rolling {
            self.roll_spritesheets
                .get_mut(&self.direction)
                .unwrap()
                .update();
        }

        GAME().camera.target = self.rect.get_center();
    }

    pub fn draw(&mut self) {
        let center = self.rect.get_center();

        /* ----------------------------- Drawing sprite ----------------------------- */
        if self.rolling {
            self.roll_spritesheets.get(&self.direction).unwrap().draw(
                center.x - 32.0,
                center.y - 32.0,
                64.0,
            );
        } else {
            self.move_spritesheets.get(&self.direction).unwrap().draw(
                center.x - 32.0,
                center.y - 32.0,
                64.0,
            );
        }

        self.draw_melee();
    }

    pub fn draw_ui(&self) {
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
