use super::gen::Objects;
use crate::scenes::objects::shapes::rect::Rect;

const OBJECTS: [&str; 4] = ["##\n##", "0##\n###", "0#\n##", "0#\n##\n#0"];

pub fn post(room: &mut [Vec<Objects>], rects: &[Rect], doors: &[(usize, usize)]) {}
