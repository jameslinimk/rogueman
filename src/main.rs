mod scenes;
mod camera;

use macroquad::prelude::*;
use crate::scenes::game::{GameScene, GAME};
use crate::scenes::object::Object;

fn config() -> Conf {
    Conf {
        window_title: "Rust game".to_owned(),
        window_width: 1280_i32,
        window_height: 720_i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    loop {
        GAME().update();
        GAME().draw();

        next_frame().await;
    }
}
