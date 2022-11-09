use lazy_static::lazy_static;

use super::gen::Objects;
use crate::scenes::objects::shapes::rect::Rect;
use crate::util::random_array;

macro_rules! objects {
    ($size: expr, $( $object: expr ), *) => {
		[
			$(
				{
					let mut vec = [[Objects::Air; $size]; $size];
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
    static ref SMALL_OBJECTS: [[[Objects; 2]; 2]; 4] =
        objects!(2, "##\n##", "#0\n##", "##\n00", "#0\n00");
    static ref MEDIUM_OBJECTS: [[[Objects; 3]; 3]; 4] =
        objects!(3, "###\n###", "##0\n###", "##0\n0##", "##0\n#0#");
}

pub fn post(room: &mut [Vec<Objects>], rects: &[Rect]) {
    println!("SMALL_OBJECTS: {:?}", random_array(&*SMALL_OBJECTS));
    println!("MEDIUM_OBJECTS: {:?}", random_array(&*MEDIUM_OBJECTS));
}
