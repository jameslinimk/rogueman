mod camera;
mod scenes;
mod util;

use crate::camera::CAMERA;
use crate::scenes::game::GAME;
use crate::scenes::object::Object;
use macroquad::prelude::*;

fn config() -> Conf {
    Conf {
        window_title: "Rust game".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    CAMERA().update_camera();

    loop {
        GAME().update();
        GAME().draw();

        next_frame().await;
    }
}
