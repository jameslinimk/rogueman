use macroquad::{prelude::YELLOW, time::get_frame_time};

use crate::{
    scenes::{
        game::GAME,
        object::{obj_id, IDObject, Object},
    },
    util::{angle, project},
};

use super::shapes::rect::Rect;

pub(crate) struct TestObj {
    rect: Rect,
    speed: f32,
    id: u32,
}
impl TestObj {
    pub fn new() -> TestObj {
        return TestObj {
            rect: Rect::new_center(100.0, 100.0, 150.0, 150.0),
            speed: 150.0,
            id: obj_id(),
        };
    }
}
impl IDObject for TestObj {
    fn update(&mut self) {
        let angle = angle(self.rect.get_center(), GAME().player.rect.get_center());
        self.rect.set_center_vec(project(
            self.rect.get_center(),
            angle,
            self.speed * get_frame_time(),
        ))
    }

    fn draw(&mut self) {
        // self.rect.draw(YELLOW);
    }

    fn get_id(&self) -> u32 {
        return self.id;
    }
}
