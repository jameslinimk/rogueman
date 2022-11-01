use macroquad::{
    prelude::{mouse_position, RED, YELLOW},
    shapes::draw_rectangle,
    time::get_frame_time,
};

use crate::{
    scenes::{
        game::GAME,
        object::{obj_id, IDObject},
    },
    util::{angle, project, rel_mouse_pos},
};

use super::shapes::{line::Line, rect::Rect};

pub struct TestObj {
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
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.rect.draw(YELLOW);
        let test = Line::new(GAME().player.rect.get_center(), rel_mouse_pos(), 10.0);
        for p in &test.points {
            draw_rectangle(p.x, p.y, 5.0, 5.0, RED);
        }
        test.draw(RED);
    }

    fn get_id(&self) -> u32 {
        return self.id;
    }
}
