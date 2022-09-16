mod scenes;

use macroquad::prelude::*;
use crate::scenes::game::GameScene;
use crate::scenes::object::Object;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut scene = GameScene::new();

    loop {
        clear_background(RED);

        scene.update();
        scene.draw();

        next_frame().await;
    }
}
