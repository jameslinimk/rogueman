mod camera;
mod scenes;
mod util;

use crate::scenes::game::GAME;
use crate::scenes::object::Object;
use macroquad::prelude::*;
use scenes::objects::assets::load_image;
use scenes::objects::guns::GUNS;

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
    for gun in GUNS {
        load_image(gun.image_file).await;
    }

    loop {
        GAME().update();
        GAME().draw();

        next_frame().await;
    }
}
