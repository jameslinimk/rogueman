#![allow(dead_code, unused_imports)]

mod camera;
mod scenes;
mod spritesheet;
mod util;

use macroquad::prelude::{next_frame, Conf};
use scenes::game::GameScene;

use crate::scenes::game::GAME;
use crate::scenes::object::Object;

fn config() -> Conf {
    Conf {
        window_title: "Rogueman".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    GameScene::init().await;

    loop {
        GAME().update();
        GAME().draw();

        next_frame().await;
    }
}
