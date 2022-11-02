use macroquad::{
    prelude::{mouse_position, vec2, PURPLE, RED, YELLOW},
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
            rect: Rect::new_center(-500.0, 100.0, 150.0, 150.0),
            speed: 150.0,
            id: obj_id(),
        };
    }
}
impl IDObject for TestObj {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let mut test = Line::new(GAME().player.rect.get_center(), rel_mouse_pos(), 10.0);
        test.draw(RED);

        if test.touches_rect(&self.rect) {
            self.rect.draw(RED);
        } else {
            self.rect.draw(YELLOW);
        }
    }

    fn get_id(&self) -> u32 {
        return self.id;
    }
}
