use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use macroquad::prelude::load_string;
use maplit::hashmap;

use super::objects::shapes::rect::Rect;

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
        for line in room.split("\n") {
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

    return walls;
}
