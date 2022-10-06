use std::collections::HashMap;

use lazy_static::lazy_static;
use macroquad::prelude::load_string;

pub(crate) enum Objects {
    AIR,
    WALL,
}

lazy_static! {
    // TODO use maplit
    static ref OBJECT_KEYS: HashMap<&'static str, Objects> = {
        let mut map = HashMap::new();
        map.insert("#", Objects::WALL);
        map.insert(" ", Objects::AIR);
        map
    };
}

pub(crate) async fn init_rooms() {
    let txt = load_string("./assets/rooms.txt").await.unwrap();
    for room in txt.split("\n\n") {
        let data: Vec<Vec<Objects>> = vec![];
        for line in room.split("\n") {}
    }

    println!("{}", txt);
}
