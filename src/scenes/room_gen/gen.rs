use crate::scenes::{
    objects::shapes::rect::Rect,
    room_gen::{
        init::init_rects,
        util::{draw_rect, find_rect, point_valid, rand_rect},
    },
};
use derive_new::new;
use lazy_static::lazy_static;
use macroquad::prelude::load_string;
use maplit::{hashmap, hashset};
use std::{collections::HashMap, sync::Mutex};

use super::{paths::paths, util::print_room};

#[derive(Clone, Copy, Debug)]
pub enum Objects {
    AIR,
    WALL,
}

lazy_static! {
    static ref OBJECT_KEYS: HashMap<&'static str, Objects> = hashmap! {
        " " => Objects::AIR,
        "#" => Objects::WALL,
    };
    pub static ref ROOMS: Mutex<Vec<Vec<Vec<Objects>>>> = Mutex::new(vec![]);
}

pub async fn init_rooms() {
    let txt = load_string("./assets/rooms.txt").await.unwrap();
    for room in txt.split("\n\n") {
        let mut data: Vec<Vec<Objects>> = vec![];
        for line in room.lines() {
            let mut line_vec: Vec<Objects> = vec![];
            for char in line.split("") {
                let obj = OBJECT_KEYS.get(char);
                if obj.is_none() {
                    continue;
                }
                line_vec.push(*obj.unwrap());
            }
            data.push(line_vec);
        }
        ROOMS.lock().unwrap().push(data);
    }
}

pub fn load_room(room: &Vec<Vec<Objects>>) -> Vec<Rect> {
    let mut walls: Vec<Rect> = vec![];

    for (y, line) in room.iter().enumerate() {
        for (x, obj) in line.iter().enumerate() {
            match obj {
                Objects::AIR => {}
                Objects::WALL => {
                    walls.push(Rect::new(x as f32 * 30.0, y as f32 * 30.0, 30.0, 30.0));
                }
            }
        }
    }

    walls
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    VERTICAL,
    HORIZONTAL,
}
#[derive(Clone, Copy, Debug, new)]
pub struct SplitQueue {
    /// In which direction to split (or draw the line)
    pub direction: Direction,
    /// Starting x to ending x (or y)
    pub x_limits: (usize, usize),
    pub y_limits: (usize, usize),
}

fn generate_room() -> Vec<Vec<Objects>> {
    let size = 100;
    let split_limit = size / 3;

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
    room = vec![vec![Objects::AIR; size as usize]; size as usize];
    for rect in &rects {
        draw_rect(rect, &mut room);
    }

    /* -------------------------------- Pathways -------------------------------- */
    paths(&rects, size, &mut room);

    room
}

#[test]
fn test() {
    let room = generate_room();
    print_room(&room);
}
