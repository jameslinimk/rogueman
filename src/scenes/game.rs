use macroquad::color::{BLACK, WHITE};
use macroquad::window::clear_background;

use crate::camera::CAMERA;
use crate::scenes::objects::player::Player;
use crate::scenes::objects::shapes::rect::Rect;
use crate::{pub_global_variable, repeat_for_vec, Object};

use super::object::IDObject;
use super::objects::enemy::Enemy;
use super::objects::test::TestObj;

pub_global_variable!(GAME, _GAME, GameScene);

pub(crate) struct GameScene {
    pub player: Player,
    pub objects: Vec<Box<dyn IDObject>>,
    pub walls: Vec<Rect>,
    pub enemies: Vec<Enemy>,
}
impl GameScene {
    pub fn new() -> GameScene {
        GameScene {
            player: Player::new(),
            objects: vec![Box::new(TestObj::new())],
            walls: vec![Rect::new(100.0, 100.0, 50.0, 50.0)],
            enemies: vec![Enemy::new(200.0, 200.0, 10.0)],
        }
    }
}
impl Object for GameScene {
    fn update(&mut self) {
        CAMERA().update();

        repeat_for_vec!(update, self.enemies, self.objects);
        self.player.update();
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        for wall in &mut self.walls {
            wall.draw(WHITE)
        }
        repeat_for_vec!(draw, self.objects, self.enemies);
        self.player.draw();
    }
}
