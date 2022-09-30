
use macroquad::color::{BLACK, WHITE, YELLOW};
use macroquad::window::clear_background;

use crate::Object;
use crate::scenes::objects::player::Player;
use crate::scenes::objects::shapes::rect::Rect;

static mut GAME_SCENE: Option<GameScene> = None;

#[allow(non_snake_case)]
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
            walls: vec![
                Rect::new(100.0, 100.0, 50.0, 50.0)
            ]
        }
    }
}
impl Object for GameScene {
    fn update(&mut self) {
        for obj in &mut self.objects { obj.update() }
        self.player.update();
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        for obj in &mut self.objects { obj.draw() }
        for wall in &mut self.walls { wall.draw(WHITE) }
        self.player.draw();
    }
}
