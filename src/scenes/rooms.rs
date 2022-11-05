use derive_new::new;
use lazy_static::lazy_static;
use macroquad::prelude::{load_string, rand::gen_range, rand::srand};
use maplit::{hashmap, hashset};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    f32::MIN,
    fs::File,
    io::Write,
    sync::Mutex,
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::scenes::objects::enemies::astar::{astar, HashVec2};

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

#[derive(Clone, Copy, Debug)]
enum Direction {
    VERTICAL,
    HORIZONTAL,
}
#[derive(Clone, Copy, Debug, new)]
struct SplitQueue {
    /// In which direction to split (or draw the line)
    direction: Direction,
    /// Starting x to ending x (or y)
    x_limits: (usize, usize),
    y_limits: (usize, usize),
}

fn pop_random(raw: &mut Vec<SplitQueue>) -> Option<SplitQueue> {
    if raw.is_empty() {
        return None;
    }
    let i = gen_range(0, raw.len());
    let copy = raw[i];
    raw.remove(i);
    Option::from(copy)
}

macro_rules! tuple_abs_diff {
    ($tuple: ident) => {
        $tuple.0.abs_diff($tuple.1)
    };
}

macro_rules! subtract_if_not_zero {
    ($number: expr) => {
        if $number == 0 {
            $number
        } else {
            $number - 1
        }
    };
}

fn cardinals(point: &(usize, usize)) -> [(usize, usize); 4] {
    [
        (point.0, point.1 + 1),
        (point.0, subtract_if_not_zero!(point.1)),
        (point.0 + 1, point.1),
        (subtract_if_not_zero!(point.0), point.1),
    ]
}

fn point_valid(point: &(usize, usize), room: &Vec<Vec<Objects>>) -> bool {
    if point.1 >= room.len() || point.0 >= room[point.1].len() {
        return false;
    }

    match room[point.1][point.0] {
        Objects::AIR => true,
        _ => false,
    }
}

fn find_rect(
    point: (usize, usize),
    room: &Vec<Vec<Objects>>,
    explored: &mut HashSet<(usize, usize)>,
) -> Option<Rect> {
    let mut queue = vec![point];

    let mut max_left = usize::MAX;
    let mut max_right = usize::MIN;
    let mut max_top = usize::MAX;
    let mut max_bottom = usize::MIN;

    while let Some(point) = queue.pop() {
        if explored.contains(&point) {
            continue;
        }

        if !point_valid(&point, room) {
            continue;
        }

        explored.insert(point);

        max_right = point.0.max(max_right);
        max_left = point.0.min(max_left);
        max_top = point.1.min(max_top);
        max_bottom = point.1.max(max_bottom);

        for direction in &cardinals(&point) {
            queue.push(*direction);
        }
    }

    if max_right == usize::MAX
        || max_top == usize::MAX
        || max_right == usize::MIN
        || max_bottom == usize::MIN
    {
        return None;
    }

    Option::from(Rect::new(
        max_left as f32,
        max_top as f32,
        max_left.abs_diff(max_right) as f32,
        max_top.abs_diff(max_bottom) as f32,
    ))
}

fn rand_rect(rect: &mut Rect) {
    let scale = gen_range::<f32>(0.7, 0.9);

    let x_diff = (rect.width - rect.width * scale).abs();
    let y_diff = (rect.height - rect.height * scale).abs();

    rect.width = (rect.width * scale).round();
    rect.height = (rect.height * scale).round();

    rect.pos.x = (rect.pos.x + x_diff * gen_range(0.0, 1.0)).round();
    rect.pos.y = (rect.pos.y + y_diff * gen_range(0.0, 1.0)).round();
}

macro_rules! vec_remove {
    ($vector: expr, $( $value: expr ), *) => {
        $(
            match $vector.iter().position(|x| x == $value) {
                Some(index) => {
                    $vector.remove(index);
                },
                None => {}
            };
        )*
    };
}

fn generate_room() -> Vec<Vec<Objects>> {
    let size = 100;
    let split_limit = size / 3;

    let mut room = vec![vec![Objects::AIR; size as usize]; size as usize];

    let mut queue = vec![SplitQueue::new(
        if gen_range(0, 2) == 0 {
            Direction::VERTICAL
        } else {
            Direction::HORIZONTAL
        },
        (0, size - 1),
        (0, size - 1),
    )];

    while let Some(split) = pop_random(&mut queue) {
        let (major_limit, minor_limit) = match split.direction {
            Direction::VERTICAL => (split.x_limits, split.y_limits),
            Direction::HORIZONTAL => (split.y_limits, split.x_limits),
        };

        if tuple_abs_diff!(major_limit) <= split_limit {
            continue;
        }

        /* ---------------------------- Splitting parent ---------------------------- */
        let diff = tuple_abs_diff!(major_limit) as f32 * gen_range(0.3, 0.7);
        let rand_split = major_limit.0 + diff as usize;
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

        for new_mi_l in [
            (major_limit.0, rand_split - 1),
            (rand_split + 1, major_limit.1),
        ] {
            queue.push(match split.direction {
                Direction::VERTICAL => {
                    if tuple_abs_diff!(new_mi_l) < tuple_abs_diff!(new_ma_l) {
                        SplitQueue::new(Direction::HORIZONTAL, new_mi_l, new_ma_l)
                    } else {
                        SplitQueue::new(Direction::VERTICAL, new_mi_l, new_ma_l)
                    }
                }
                Direction::HORIZONTAL => {
                    if tuple_abs_diff!(new_mi_l) < tuple_abs_diff!(new_ma_l) {
                        SplitQueue::new(Direction::VERTICAL, new_ma_l, new_mi_l)
                    } else {
                        SplitQueue::new(Direction::HORIZONTAL, new_ma_l, new_mi_l)
                    }
                }
            });
        }
    }

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
        for x in rect.pos.x as usize..=rect.get_right() as usize {
            room[rect.pos.y as usize][x] = Objects::WALL;
            room[rect.get_bottom() as usize][x] = Objects::WALL;
        }
        for y in rect.pos.y as usize..=rect.get_bottom() as usize {
            room[y][rect.pos.x as usize] = Objects::WALL;
            room[y][rect.get_right() as usize] = Objects::WALL;
        }
    }

    /* -------------------------------- Pathways -------------------------------- */
    let path_width = 3;
    let mut adjacent_rects = hashmap! {};

    for (i, rect) in rects.iter().enumerate() {
        for dir in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let mut i = 0;
            let mut first_i = false;
            loop {
                if !first_i {
                    first_i = true;
                } else {
                    i += 1;
                }
                if i >= size as i32 {
                    break;
                }

                let x = if dir.0 != 0 {
                    dir.0 * i
                } else {
                    rect.get_center().x as i32
                };

                let y = if dir.1 != 0 {
                    dir.1 * i
                } else {
                    rect.get_center().y as i32
                };

                if x < 0 || y < 0 || x >= size as i32 || y >= size as i32 {
                    break;
                }

                let mut empty_explored = hashset! {};

                let r = match find_rect((x as usize, y as usize), &room, &mut empty_explored) {
                    Some(rect) => rect,
                    None => continue,
                };
                println!("r: {:?}", r);
                println!("rects.contains(&r): {:?}", rects.contains(&r));
                if rect != &r && rects.contains(&r) {
                    println!("hello");
                    let index = rects.iter().position(|rect| rect == &r).unwrap();

                    if !adjacent_rects.contains_key(&i) {
                        adjacent_rects.insert(i, vec![index]);
                    } else {
                        adjacent_rects.get_mut(&i).unwrap().push(index);
                    };

                    continue;
                }
            }
        }
        break;
    }

    println!("adjacent_rects: {:?}", adjacent_rects);

    room
}

#[test]
fn test() {
    let room = generate_room();
    print_room(&room);
}

// /* -------------------------- Combining small rects ------------------------- */
// let mut sim_rects = vec![];
// let mut explored_rects = vec![];
// for rect in &rects {
//     if explored_rects.contains(rect) {
//         continue;
//     }
//     explored_rects.push(*rect);

//     let wi = rect.width - 2.0;
//     let he = rect.height - 2.0;

//     if wi < split_limit as f32 {
//         for point in [(rect.pos.x + 2.0) as usize, (rect.pos.x - 2.0) as usize] {
//             let mut empty_explored = hashset! {};
//             let r = find_rect((point, rect.pos.y as usize), &room, &mut empty_explored);

//             if explored_rects.contains(&r) || rect == &r {
//                 continue;
//             }

//             explored_rects.push(r);

//             if rect.pos.y == r.pos.y && rect.height == r.height {
//                 sim_rects.push((*rect, r));
//             }
//         }
//     }

//     if he < split_limit as f32 {
//         for point in [(rect.pos.y + 2.0) as usize, (rect.pos.y - 2.0) as usize] {
//             let mut empty_explored = hashset! {};
//             let r = find_rect((rect.pos.x as usize, point), &room, &mut empty_explored);

//             if explored_rects.contains(&r) || rect == &r {
//                 continue;
//             }

//             explored_rects.push(r);

//             if rect.pos.x == r.pos.x && rect.width == r.width {
//                 sim_rects.push((*rect, r));
//             }
//         }
//     }
// }

// for (r1, r2) in &sim_rects {
//     let new_rect = if r1.height == r2.height {
//         Rect::new(
//             r1.pos.x.min(r2.pos.x),
//             r1.pos.y.min(r2.pos.y),
//             (r1.pos.x.min(r2.pos.x) - r1.get_right().max(r2.get_right())).abs(),
//             r1.height,
//         )
//     } else {
//         Rect::new(
//             r1.pos.x.min(r2.pos.x),
//             r1.pos.y.min(r2.pos.y),
//             r1.width,
//             (r1.pos.y.min(r2.pos.y) - r1.get_bottom().max(r2.get_bottom())).abs(),
//         )
//     };

//     vec_remove!(rects, r1, r2);
//     rects.push(new_rect);
// }
