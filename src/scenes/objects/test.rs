use super::shapes::rect::Rect;
use crate::{
    scenes::object::{obj_id, IDObject},
    util::rel_mouse_pos,
};
use derive_new::new;
use macroquad::prelude::{RED, YELLOW};

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
        if self.rect.touches_point(&rel_mouse_pos()) {
            self.rect.draw(RED);
        } else {
            self.rect.draw(YELLOW);
        }
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
