use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{is_key_down, is_key_pressed, Color, KeyCode};
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
        GameScene {
            player: Player::new(),
            objects: vec![Box::new(TestObj::new())],
            walls: vec![Rect::new(100.0, 100.0, 500.0, 50.0)],
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

        for wall in &mut self.walls {
            wall.draw(WHITE)
        }
        repeat_for_vec!(draw, self.objects, self.enemies);
        self.player.draw();
    }
}
