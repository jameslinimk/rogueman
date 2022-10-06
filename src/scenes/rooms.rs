use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use macroquad::prelude::load_string;
use maplit::hashmap;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Objects {
    AIR,
    WALL,
}

lazy_static! {
    static ref OBJECT_KEYS: HashMap<&'static str, Objects> = hashmap! {
        " " => Objects::AIR,
        "#" => Objects::WALL,
    };
    pub(crate) static ref ROOMS: Mutex<Vec<Vec<Vec<Objects>>>> = Mutex::new(vec![]);
}

pub(crate) async fn init_rooms() {
    let txt = load_string("./assets/rooms.txt").await.unwrap();
    for room in txt.split("\n\n") {
        let mut data: Vec<Vec<Objects>> = vec![];
        for line in room.split("\n") {
            let mut line_vec: Vec<Objects> = vec![];
            for char in line.trim().split("") {
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
