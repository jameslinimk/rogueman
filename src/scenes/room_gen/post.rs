use lazy_static::lazy_static;
use macroquad::rand::gen_range;

use super::gen::Objects;
use crate::scenes::objects::shapes::rect::Rect;
use crate::scenes::room_gen::util::{print_room, rotate_array};
use crate::util::random_array;

macro_rules! objects {
    ($size: expr, $( $object: expr ), *) => {
		[
			$(
				{
					let mut vec = vec![vec![Objects::Air; $size]; $size];
					for (y, line) in $object.lines().enumerate() {
						for (x, char) in line.trim().chars().enumerate() {
							vec[y][x] = if char == '#' {
								Objects::Wall
							} else {
								Objects::Air
							};
						}
					}
					vec
				},
			)*
		]
    };
}

lazy_static! {
    static ref SMALL_OBJECTS: [Vec<Vec<Objects>>; 4] = objects!(
        2,
        "##
         ##",
        "#
         ##",
        "##",
        "#"
    );
    static ref MEDIUM_OBJECTS: [Vec<Vec<Objects>>; 7] = objects!(
        3,
        "###
         ###",
        "##
         ###",
        "##
         0##",
        "###
         # #",
        "###
         ##
         #  ",
        "0##
         0##
         ## ",
        "###
         # #
         ###"
    );
    static ref LARGE_OBJECTS: [Vec<Vec<Objects>>; 4] = objects!(
        5,
        "
         #
         #  ##
         ##  #
         ##  #",
        "##  #
         0000#
        
         #  ##
         #   #",
        "##
         #   #
         #   #
         #
         #    ",
        "#####
         #
         0000#
         000##
         #    "
    );
}

fn complete_random(array: &[Vec<Vec<Objects>>]) -> Vec<Vec<Objects>> {
    let mut random = random_array(array).to_vec();
    for _ in 0..gen_range(0, 3) {
        rotate_array(&mut random);
    }
    random
}

fn paste_object(
    x: usize,
    y: usize,
    object: &[Vec<Objects>],
    rect: &Rect,
    room: &mut [Vec<Objects>],
) {
    let x_offset = rect.pos.x as usize;
    let y_offset = rect.pos.y as usize;

    for (obj_y, row) in object.iter().enumerate() {
        for (obj_x, cell) in row.iter().enumerate() {
            if cell == &Objects::Wall {
                room[y + obj_y + y_offset][x + obj_x + x_offset] = Objects::Wall;
            }
        }
    }
}

#[test]
fn test() {
    for _ in 0..15 {
        let random = complete_random(&*LARGE_OBJECTS);
        println!();
        print_room(&random);
    }
}

fn gen_battle_room(room: &mut [Vec<Objects>], rect: &Rect) {
    paste_object(1, 1, &complete_random(&*LARGE_OBJECTS), rect, room);
}

pub fn post(room: &mut [Vec<Objects>], rects: &mut [Rect]) {
    for rect in rects.iter_mut() {
        gen_battle_room(room, rect);
    }
}
