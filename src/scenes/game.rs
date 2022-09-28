use std::borrow::BorrowMut;
use macroquad::color::{BLACK};
use macroquad::window::clear_background;
use crate::camera::CAMERA;
use crate::Object;
use crate::scenes::objects::player::Player;
use crate::scenes::objects::shapes::rect::Rect;

static mut GAME_SCENE: Option<GameScene> = None;

pub(crate) fn GAME() -> &'static mut GameScene {
    unsafe {
        if GAME_SCENE.is_none() { GAME_SCENE = Option::from(GameScene::new()) }
        return GAME_SCENE.as_mut().unwrap()
    }
}

pub(crate) struct GameScene {
    pub player: Player,
    pub objects: Vec<Box<dyn Object>>,
    pub walls: Vec<Rect>
}
impl GameScene {
    pub fn new() -> GameScene {
        GameScene {
            player: Player::new(),
            objects: vec![],
            walls: vec![]
        }
    }
}
impl Object for GameScene {
    fn update(&mut self) {
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        for obj in &mut self.objects {
            obj.update();
            obj.draw();
        }
        self.player.update();
        self.player.draw();
    }
}
