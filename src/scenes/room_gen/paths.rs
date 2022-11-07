use super::gen::Objects;
use crate::scenes::objects::shapes::rect::Rect;
use macroquad::prelude::vec2;
use maplit::hashmap;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

type AdjacentRects = HashMap<usize, Vec<(usize, (i32, i32))>>;
fn adjacent_rects(rects: &[Rect], size: usize) -> AdjacentRects {
    let mut adjacent_rects: AdjacentRects = hashmap! {};
    for (rect_index, rect) in rects.iter().enumerate() {
        let mut already_adjacent = vec![];
        for dir in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            for i in 0..size / 2 {
                let x = if dir.0 != 0 {
                    rect.get_center().x as i32 + dir.0 * i as i32
                } else {
                    rect.get_center().x as i32
                };

                let y = if dir.1 != 0 {
                    rect.get_center().y as i32 + dir.1 * i as i32
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

                    // Check if the rect-r connection already exists
                    if let Some(vec) = adjacent_rects.get(&r_index) {
                        if vec.iter().any(|x| x.0 == rect_index) {
                            break;
                        }
                    }

                    if let Vacant(entry) = adjacent_rects.entry(rect_index) {
                        entry.insert(vec![(r_index, *dir)]);
                    } else {
                        adjacent_rects
                            .get_mut(&rect_index)
                            .unwrap()
                            .push((r_index, *dir));
                    };

                    break;
                }
            }
        }
    }

    adjacent_rects
}

pub fn paths(rects: &[Rect], size: usize, room: &mut [Vec<Objects>]) {
    let path_size = 3;
    let half_path_size = path_size / 2 + 1;

    let adjacents = adjacent_rects(rects, size);
    for (rect_index, rs) in &adjacents {
        let rect = rects[*rect_index];
        for (r_index, dir) in rs {
            let r = rects[*r_index];

            let start = match dir {
                (0, 1) => (rect.get_center().x as i32, rect.get_bottom() as i32),
                (0, -1) => (rect.get_center().x as i32, rect.get_top() as i32),
                (1, 0) => (rect.get_right() as i32, rect.get_center().y as i32),
                (-1, 0) => (rect.get_left() as i32, rect.get_center().y as i32),
                _ => panic!(),
            };
            let end = match dir {
                (0, -1) => (rect.get_center().x as i32, r.get_bottom() as i32),
                (0, 1) => (rect.get_center().x as i32, r.get_top() as i32),
                (-1, 0) => (r.get_right() as i32, rect.get_center().y as i32),
                (1, 0) => (r.get_left() as i32, rect.get_center().y as i32),
                _ => panic!(),
            };

            // TODO Fix path going over the side of a rect

            let mut pos = start;
            loop {
                if pos == end || pos.0 < 1 || pos.1 < 1 {
                    break;
                }

                if dir.0 == 0 {
                    room[pos.1 as usize][pos.0 as usize + half_path_size] = Objects::Wall;
                    room[pos.1 as usize][pos.0 as usize - half_path_size] = Objects::Wall;
                } else {
                    room[pos.1 as usize + half_path_size][pos.0 as usize] = Objects::Wall;
                    room[pos.1 as usize - half_path_size][pos.0 as usize] = Objects::Wall;
                }

                pos.0 += dir.0;
                pos.1 += dir.1;
            }
        }
    }
}
