use macroquad::prelude::rand::ChooseRandom;
use macroquad::prelude::{clear_background, WHITE};

use super::dungeon_manager::Manager;
use super::object::IDObject;
use super::objects::assets::load_image;
use super::objects::enemies::enemy::Enemy;
use super::objects::items::guns::GUNS;
use super::objects::objects_enum::Objects;
use super::objects::player::main::Player;
use super::objects::test::TestObj;
use super::room_gen::gen::{generate_room, load_walls, Objects as RoomObjects};
use crate::camera::Camera;
use crate::scenes::objects::shapes::rect::Rect;
use crate::util::hex;
use crate::{pub_global_variable, repeat_for_vec, repeat_function, Object};

pub_global_variable!(GAME, _GAME, GameScene);

pub struct GameScene {
    pub player: Player,
    pub objects: Vec<Objects>,
    pub walls: Vec<Rect>,
    pub manager: Manager,
    pub enemies: Vec<Enemy>,
    pub camera: Camera,
}
impl GameScene {
    pub fn new() -> GameScene {
        let manager = generate_room();
        GameScene {
            player: Player::new(),
            objects: vec![Objects::from(TestObj::new())],
            manager: manager.clone(),
            walls: load_walls(&manager.room),
            enemies: vec![Enemy::new(200.0, 200.0, 10.0)],
            camera: Camera::new(),
        }
    }

    pub async fn init() {
        load_image("./assets/guns/border.png").await;
        for gun in GUNS {
            load_image(gun.image_file).await;
        }
        Player::init().await;
    }

    pub fn remove_object(&mut self, id: u32) {
        if let Some(index) = self.objects.iter().position(|x| x.get_id() == id) {
            self.objects.remove(index);
        }
    }

    pub fn remove_enemy(&mut self, id: u32) {
        if let Some(index) = self.enemies.iter().position(|x| x.get_id() == id) {
            self.enemies.remove(index);
        }
    }
}
impl Object for GameScene {
    fn update(&mut self) {
        repeat_for_vec!(update, self.enemies, self.objects);
        repeat_function!(update, self.player, self.camera);
    }

    fn draw(&mut self) {
        clear_background(hex("#313639"));

        repeat_function!(draw, self.player);
        for wall in &mut self.walls {
            wall.draw(WHITE)
        }
        repeat_for_vec!(draw, self.objects, self.enemies);
        self.player.draw_ui();
    }
}
