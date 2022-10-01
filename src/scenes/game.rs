use macroquad::color::{BLACK, WHITE};
use macroquad::window::clear_background;

use crate::camera::CAMERA;
use crate::scenes::objects::player::Player;
use crate::scenes::objects::shapes::rect::Rect;
use crate::{pub_global_variable, Object};

use super::object::IDObject;
use super::objects::test::TestObj;

pub_global_variable!(GAME, _GAME, GameScene);

pub(crate) struct GameScene {
    pub player: Player,
    pub objects: Vec<Box<dyn IDObject>>,
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

    pub fn remove_obj(&mut self, id: u32) {
        let index = match self.objects.iter().position(|x| x.get_id() == id) {
            Some(index) => index,
            None => return,
        };
        self.objects.remove(index);
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
