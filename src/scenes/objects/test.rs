use derive_new::new;
use macroquad::prelude::{RED, YELLOW};

use crate::{
    scenes::{
        game::GAME,
        object::{obj_id, IDObject},
    },
    util::rel_mouse_pos,
};

use super::shapes::{line::Line, rect::Rect};

#[derive(new)]
pub struct TestObj {
    #[new(value = "Rect::new_center(-500.0, 100.0, 150.0, 150.0)")]
    rect: Rect,
    #[new(value = "obj_id()")]
    id: u32,
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
