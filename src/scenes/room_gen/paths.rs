use std::collections::HashMap;

use macroquad::prelude::vec2;
use maplit::hashmap;

use std::collections::hash_map::Entry::Vacant;

use crate::scenes::{object::Object, objects::shapes::rect::Rect};

use super::gen::Objects;

fn adjacent_rects(rects: &Vec<Rect>, size: usize) -> HashMap<usize, Vec<usize>> {
    let mut adjacent_rects = hashmap! {};
    for (rect_index, rect) in rects.iter().enumerate() {
        let mut already_adjacent = vec![];
        for dir in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let mut i = 0;
            let mut first_i = false;
            loop {
                if !first_i {
                    first_i = true;
                } else {
                    i += 1;
                }
                if i >= size as i32 / 2 {
                    break;
                }

                let x = if dir.0 != 0 {
                    rect.get_center().x as i32 + dir.0 * i
                } else {
                    rect.get_center().x as i32
                };

                let y = if dir.1 != 0 {
                    rect.get_center().y as i32 + dir.1 * i
                } else {
                    rect.get_center().y as i32
                };

                if x < 0 || y < 0 || x >= size as i32 || y >= size as i32 {
                    break;
                }

                let r = match rects
                    .iter()
                    .position(|r| r.touches_point(&vec2(x as f32, y as f32)))
                {
                    Some(rect) => rects[rect],
                    None => continue,
                };

                if rect != &r && rects.contains(&r) && !already_adjacent.contains(&r) {
                    already_adjacent.push(r);
                    let r_index = rects.iter().position(|rect| rect == &r).unwrap();

                    if let Vacant(entry) = adjacent_rects.entry(rect_index) {
                        entry.insert(vec![r_index]);
                    } else {
                        adjacent_rects.get_mut(&rect_index).unwrap().push(r_index);
                    };

                    break;
                }
            }
        }
    }

    adjacent_rects
}

pub fn paths(rects: &Vec<Rect>, size: usize, room: &mut Vec<Vec<Objects>>) {
    let adjacents = adjacent_rects(rects, size);
}
