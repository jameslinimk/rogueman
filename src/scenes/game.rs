use lazy_static::lazy_static;
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{is_key_down, is_key_pressed, Color, KeyCode};
use macroquad::rand::{gen_range, ChooseRandom};
use macroquad::window::clear_background;

use crate::camera::{Camera, ShakeConfig};
use crate::scenes::objects::player::Player;
use crate::scenes::objects::shapes::rect::Rect;
use crate::util::hex;
use crate::{pub_global_variable, repeat_for_vec, Object};

use super::object::IDObject;
use super::objects::assets::load_image;
use super::objects::enemies::enemy::Enemy;
use super::objects::items::guns::GUNS;
use super::objects::objects::Objects;
use super::objects::test::TestObj;
use super::rooms::{Objects as RoomObjects, ROOMS};

pub_global_variable!(GAME, _GAME, GameScene);

pub(crate) struct GameScene {
    pub player: Player,
    pub objects: Vec<Objects>,
    pub walls: Vec<Rect>,
    pub enemies: Vec<Enemy>,
    pub camera: Camera,
}
impl GameScene {
    pub fn new() -> GameScene {
        let mut walls: Vec<Rect> = vec![];

        let rooms = ROOMS.lock().unwrap();
        let rand_room = rooms.choose().unwrap();

        for (y, line) in rand_room.iter().enumerate() {
            for (x, obj) in line.iter().enumerate() {
                match obj {
                    RoomObjects::AIR => {}
                    RoomObjects::WALL => {
                        walls.push(Rect::new(x as f32 * 30.0, y as f32 * 30.0, 30.0, 30.0));
                    }
                }
            }
        }

        GameScene {
            player: Player::new(),
            objects: vec![],
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
        clear_background(hex("#313639"));

        self.player.draw();
        repeat_for_vec!(draw, self.objects, self.enemies);
        for wall in &mut self.walls {
            wall.draw(WHITE)
        }
    }
}
