use super::gen::{Objects, SplitQueue};
use crate::scenes::objects::shapes::rect::Rect;
use macroquad::prelude::rand::gen_range;
use std::collections::HashSet;

pub fn print_room(rooms: &Vec<Vec<Objects>>) {
    for row in rooms {
        for cell in row {
            let sym = match cell {
                Objects::Air => "⬜",
                Objects::Wall => "⬛",
            };
            print!("{}", sym);
        }
        println!();
    }
}

pub fn pop_random(raw: &mut Vec<SplitQueue>) -> Option<SplitQueue> {
    if raw.is_empty() {
        return None;
    }
    let i = gen_range(0, raw.len());
    let copy = raw[i];
    raw.remove(i);
    Option::from(copy)
}

#[macro_export]
macro_rules! tuple_abs_diff {
    ($tuple: ident) => {
        $tuple.0.abs_diff($tuple.1)
    };
}

#[macro_export]
macro_rules! subtract_if_not_zero {
    ($number: expr) => {
        if $number == 0 {
            $number
        } else {
            $number - 1
        }
    };
}

pub fn cardinals(point: &(usize, usize)) -> [(usize, usize); 4] {
    [
        (point.0, point.1 + 1),
        (point.0, subtract_if_not_zero!(point.1)),
        (point.0 + 1, point.1),
        (subtract_if_not_zero!(point.0), point.1),
    ]
}

pub fn point_valid(point: &(usize, usize), room: &Vec<Vec<Objects>>) -> bool {
    if point.1 >= room.len() || point.0 >= room[point.1].len() {
        return false;
    }

    matches!(room[point.1][point.0], Objects::Air)
}

pub fn find_rect(
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

pub fn rand_rect(rect: &mut Rect) {
    let scale = gen_range::<f32>(0.7, 0.9);

    let x_diff = (rect.width - rect.width * scale).abs();
    let y_diff = (rect.height - rect.height * scale).abs();

    rect.width = (rect.width * scale).round();
    rect.height = (rect.height * scale).round();

    rect.pos.x = (rect.pos.x + x_diff * gen_range(0.0, 1.0)).round();
    rect.pos.y = (rect.pos.y + y_diff * gen_range(0.0, 1.0)).round();
}

#[macro_export]
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

pub fn draw_rect(rect: &Rect, room: &mut [Vec<Objects>]) {
    for x in rect.pos.x as usize..=rect.get_right() as usize {
        room[rect.pos.y as usize][x] = Objects::Wall;
        room[rect.get_bottom() as usize][x] = Objects::Wall;
    }
    for row in room
        .iter_mut()
        .take(rect.get_bottom() as usize + 1)
        .skip(rect.pos.y as usize)
    {
        row[rect.pos.x as usize] = Objects::Wall;
        row[rect.get_right() as usize] = Objects::Wall;
    }
}
