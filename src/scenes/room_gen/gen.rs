use std::collections::HashMap;
use std::sync::Mutex;

use derive_new::new;
use lazy_static::lazy_static;
use macroquad::prelude::load_string;
use maplit::{hashmap, hashset};

use super::paths::paths;
use super::post::post;
use super::util::print_room;
use crate::scenes::objects::shapes::rect::Rect;
use crate::scenes::room_gen::init::init_rects;
use crate::scenes::room_gen::util::{draw_rect, find_rect, point_valid, rand_rect};
use crate::util::SQUARE_SIZE;

#[derive(Clone, Copy, Debug)]
pub enum Objects {
    Air,
    Wall,
}

pub fn load_walls(room: &[Vec<Objects>]) -> Vec<Rect> {
    let mut walls: Vec<Rect> = vec![];

    for (y, line) in room.iter().enumerate() {
        for (x, obj) in line.iter().enumerate() {
            match obj {
                Objects::Air => {}
                Objects::Wall => {
                    walls.push(Rect::new(
                        x as f32 * SQUARE_SIZE,
                        y as f32 * SQUARE_SIZE,
                        SQUARE_SIZE,
                        SQUARE_SIZE,
                    ));
                }
            }
        }
    }

    walls
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Vertical,
    Horizontal,
}
#[derive(Clone, Copy, Debug, new)]
pub struct SplitQueue {
    /// In which direction to split (or draw the line)
    pub direction: Direction,
    /// Starting x to ending x (or y)
    pub x_limits: (usize, usize),
    pub y_limits: (usize, usize),
}

pub fn generate_room() -> Vec<Vec<Objects>> {
    let size = 100;
    let split_limit = size / 3;

    /* ------------------------------- Init rects ------------------------------- */
    let mut room = init_rects(size, split_limit);

    /* -------------------------- Detecting rectangles -------------------------- */
    let mut rects = vec![];
    let mut explored = hashset! {};
    for (y, row) in room.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if !point_valid(&(x, y), &room) || explored.contains(&(x, y)) {
                continue;
            }

            let rect = find_rect((x, y), &room, &mut explored).unwrap();

            rects.push(rect);
        }
    }

    /* ---------------------------- Random sub rects ---------------------------- */
    for rect in &mut rects {
        rand_rect(rect);
    }

    /* ------------------------------ Drawing rects ----------------------------- */
    room = vec![vec![Objects::Air; size]; size];
    for rect in &rects {
        draw_rect(rect, &mut room);
    }

    /* ---------------------------- Pathways and post --------------------------- */
    let doors = paths(&rects, size, &mut room);
    post(&mut room, &rects);

    room
}

#[test]
fn test() {
    let room = generate_room();
    print_room(&room);
}
