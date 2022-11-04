use derive_new::new;
use lazy_static::lazy_static;
use macroquad::{
    prelude::load_string,
    rand::{gen_range, srand},
};
use maplit::hashmap;
use std::{collections::HashMap, sync::Mutex, time::SystemTime};

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

fn print_room(rooms: &Vec<Vec<Objects>>) {
    for (y, row) in rooms.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let sym = match cell {
                Objects::AIR => "⬜",
                Objects::WALL => "⬛",
            };
            print!("{}", sym);
        }
        println!();
    }
}

enum Direction {
    VERTICAL,
    HORIZONTAL,
}
#[derive(new)]
struct SplitQueue {
    /// In which direction to split (or draw the line)
    direction: Direction,
    /// Starting x to ending x (or y)
    x_limits: (usize, usize),
    y_limits: (usize, usize),
}

#[test]
fn generate_room() {
    let size = 100;
    let min_room_size = size / 10;

    srand(1253563463456);

    let mut room = vec![vec![Objects::AIR; size as usize]; size as usize];

    let mut queue = vec![SplitQueue::new(
        Direction::VERTICAL,
        (0, size - 1),
        (0, size - 1),
    )];
    loop {
        let s = queue.pop();
        if s.is_none() {
            break;
        }
        let split = s.unwrap();

        let (major_limit, minor_limit) = match split.direction {
            Direction::VERTICAL => (split.x_limits, split.y_limits),
            Direction::HORIZONTAL => (split.y_limits, split.x_limits),
        };

        if major_limit.0.abs_diff(major_limit.1) < min_room_size {
            continue;
        }

        /* ---------------------------- Splitting parent ---------------------------- */
        let rand_split = gen_range(major_limit.0 + min_room_size, major_limit.1 - min_room_size);

        for i in minor_limit.0..=minor_limit.1 {
            match split.direction {
                Direction::VERTICAL => {
                    room[i][rand_split] = Objects::WALL;
                }
                Direction::HORIZONTAL => {
                    room[rand_split][i] = Objects::WALL;
                }
            }
        }

        /* -------------------------- Calculating children -------------------------- */
        let new_ma_l = minor_limit;

        // FIXME Debug this
        for new_mi_l in [
            (major_limit.0, rand_split - 1),
            (rand_split + 1, major_limit.1),
        ] {
            queue.push(match split.direction {
                Direction::VERTICAL => SplitQueue::new(Direction::HORIZONTAL, new_mi_l, new_ma_l),
                Direction::HORIZONTAL => SplitQueue::new(Direction::HORIZONTAL, new_ma_l, new_mi_l),
            });
        }
    }

    print_room(&room);
}
