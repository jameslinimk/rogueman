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
						for (x, char) in line.chars().enumerate() {
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
    static ref SMALL_OBJECTS: [Vec<Vec<Objects>>; 4] =
        objects!(2, "##\n##", "#0\n##", "##\n00", "#0\n00");
    static ref MEDIUM_OBJECTS: [Vec<Vec<Objects>>; 7] = objects!(
        3,
        "###\n###",
        "##0\n###",
        "##0\n0##",
        "###\n#0#",
        "###\n##\n#",
        "0##\n0##\n##0",
        "###\n#0#\n###"
    );
}

fn complete_random(array: &[Vec<Vec<Objects>>]) -> Vec<Vec<Objects>> {
    let mut random = random_array(array).to_vec();
    for _ in 0..gen_range(0, 3) {
        rotate_array(&mut random);
    }
    random
}

#[test]
fn test() {
    for _ in 0..15 {
        let random = complete_random(&*MEDIUM_OBJECTS);
        println!();
        print_room(&random);
    }
}

pub fn post(room: &mut [Vec<Objects>], rects: &[Rect]) {
    println!("SMALL_OBJECTS: {:?}", random_array(&*SMALL_OBJECTS));
    println!("MEDIUM_OBJECTS: {:?}", random_array(&*MEDIUM_OBJECTS));
}
