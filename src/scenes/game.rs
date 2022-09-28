use std::borrow::BorrowMut;
use macroquad::color::{BLACK};
use macroquad::window::clear_background;
use crate::Object;
use crate::scenes::objects::player::Player;

pub(crate) struct GameScene {
    player: Player
}
impl GameScene {
    pub fn new() -> GameScene {
        GameScene {
            player: Player::new()
        }
    }
}
impl Object for GameScene {
    fn update(&mut self) {
        self.player.update();
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        self.player.draw();
    }
}
