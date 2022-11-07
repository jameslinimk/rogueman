use macroquad::rand::gen_range;

use super::gen::{Direction, Objects, SplitQueue};
use super::util::pop_random;
use crate::tuple_abs_diff;

pub fn init_rects(size: usize, split_limit: usize) -> Vec<Vec<Objects>> {
    let mut room = vec![vec![Objects::Air; size]; size];

    let mut queue = vec![SplitQueue::new(
        if gen_range(0, 2) == 0 {
            Direction::Vertical
        } else {
            Direction::Horizontal
        },
        (0, size - 1),
        (0, size - 1),
    )];

    while let Some(split) = pop_random(&mut queue) {
        let (major_limit, minor_limit) = match split.direction {
            Direction::Vertical => (split.x_limits, split.y_limits),
            Direction::Horizontal => (split.y_limits, split.x_limits),
        };

        if tuple_abs_diff!(major_limit) <= split_limit {
            continue;
        }

        /* ---------------------------- Splitting parent ---------------------------- */
        let diff = tuple_abs_diff!(major_limit) as f32 * gen_range(0.3, 0.7);
        let rand_split = major_limit.0 + diff as usize;
        for i in minor_limit.0..=minor_limit.1 {
            match split.direction {
                Direction::Vertical => {
                    room[i][rand_split] = Objects::Wall;
                }
                Direction::Horizontal => {
                    room[rand_split][i] = Objects::Wall;
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
                Direction::Vertical => {
                    if tuple_abs_diff!(new_mi_l) < tuple_abs_diff!(new_ma_l) {
                        SplitQueue::new(Direction::Horizontal, new_mi_l, new_ma_l)
                    } else {
                        SplitQueue::new(Direction::Vertical, new_mi_l, new_ma_l)
                    }
                }
                Direction::Horizontal => {
                    if tuple_abs_diff!(new_mi_l) < tuple_abs_diff!(new_ma_l) {
                        SplitQueue::new(Direction::Vertical, new_ma_l, new_mi_l)
                    } else {
                        SplitQueue::new(Direction::Horizontal, new_ma_l, new_mi_l)
                    }
                }
            });
        }
    }

    room
}
