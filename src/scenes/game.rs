use lazy_static::lazy_static;
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{is_key_down, is_key_pressed, Color, KeyCode};
use macroquad::rand::gen_range;
use macroquad::window::clear_background;

use crate::camera::{Camera, ShakeConfig};
use crate::scenes::objects::player::Player;
use crate::scenes::objects::shapes::rect::Rect;
use crate::util::hex_to_color;
use crate::{pub_global_variable, repeat_for_vec, Object};

use super::object::IDObject;
use super::objects::assets::load_image;
use super::objects::enemy::Enemy;
use super::objects::guns::GUNS;
use super::objects::test::TestObj;
use super::rooms::{Objects, ROOMS};

pub_global_variable!(GAME, _GAME, GameScene);

pub(crate) struct GameScene {
    pub player: Player,
    pub objects: Vec<Box<dyn IDObject>>,
    pub walls: Vec<Rect>,
    pub enemies: Vec<Enemy>,
    pub camera: Camera,
}
impl GameScene {
    pub fn new() -> GameScene {
        let mut walls: Vec<Rect> = vec![];

        let rooms = ROOMS.lock().unwrap();
        let room_index = gen_range(0, rooms.len());

        for (y, line) in rooms[room_index].iter().enumerate() {
            for (x, obj) in line.iter().enumerate() {
                match obj {
                    Objects::AIR => {}
                    Objects::WALL => {
                        walls.push(Rect::new(x as f32 * 30.0, y as f32 * 30.0, 30.0, 30.0));
                    }
                }
            }
        }

        GameScene {
            player: Player::new(),
            objects: vec![Box::new(TestObj::new())],
            walls,
            enemies: vec![Enemy::new(200.0, 200.0, 10.0)],
            camera: Camera::new(),
        }
    }

    pub async fn init(&self) {
        load_image("./assets/guns/border.png").await;
        for gun in GUNS {
            load_image(gun.image_file).await;
        }
    }
}
impl Object for GameScene {
    fn update(&mut self) {
        repeat_for_vec!(update, self.enemies, self.objects);
        self.player.update();
        self.camera.update();
    }

    fn draw(&mut self) {
        clear_background(hex_to_color("#313639"));

        self.player.draw();
        repeat_for_vec!(draw, self.objects, self.enemies);
        for wall in &mut self.walls {
            wall.draw(WHITE)
        }
    }
}
