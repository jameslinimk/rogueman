use macroquad::color::{BLACK, WHITE};
use macroquad::window::clear_background;

use crate::camera::CAMERA;
use crate::scenes::objects::player::Player;
use crate::scenes::objects::shapes::rect::Rect;
use crate::{pub_global_variable, Object};

use super::objects::test::TestObj;

pub_global_variable!(GAME, _GAME, GameScene);

pub(crate) struct GameScene {
    pub player: Player,
    pub objects: Vec<Box<dyn Object>>,
    pub walls: Vec<Rect>,
}
impl GameScene {
    pub fn new() -> GameScene {
        GameScene {
            player: Player::new(),
            objects: vec![Box::new(TestObj::new())],
            walls: vec![Rect::new(100.0, 100.0, 50.0, 50.0)],
        }
    }
}
impl Object for GameScene {
    fn update(&mut self) {
        CAMERA().update();

        for obj in &mut self.objects {
            obj.update()
        }
        self.player.update();
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        for obj in &mut self.objects {
            obj.draw()
        }
        for wall in &mut self.walls {
            wall.draw(WHITE)
        }
        self.player.draw();
    }
}
