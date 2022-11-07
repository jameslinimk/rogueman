use super::{
    gen::{Direction, Objects, SplitQueue},
    util::pop_random,
};
use crate::tuple_abs_diff;
use macroquad::rand::gen_range;

pub fn init_rects(size: usize, split_limit: usize) -> Vec<Vec<Objects>> {
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

    room
}
