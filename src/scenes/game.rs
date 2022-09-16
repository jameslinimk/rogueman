use crate::Object;
use crate::scenes::objects::player::Player;

pub(crate)  struct GameScene {
    objects: Vec<Box<dyn Object>>
}
impl GameScene {
    pub fn new() -> GameScene {
        let mut scene = GameScene {
            objects: Vec::new()
        };

        scene.objects.push(Box::new(Player::new()));

        return scene;
    }
}
impl Object for GameScene {
    fn update(&mut self) {
    }

    fn draw(&mut self) {
        for obj in &mut self.objects {
            obj.update();
            obj.draw();
        };
    }
}
