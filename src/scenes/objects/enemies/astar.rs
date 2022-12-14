use derive_new::new;
use macroquad::prelude::Vec2;
use maplit::{hashmap, hashset};
use priority_queue::PriorityQueue;

use crate::scenes::room_gen::gen::Objects;

fn manhattan_heuristic(pos: &HashVec2, goal: &HashVec2) -> i32 {
    let x_dis = pos.x.abs_diff(goal.x) as i32;
    let y_dis = pos.y.abs_diff(goal.y) as i32;
    -(x_dis + y_dis)
}

fn pos_valid(pos: &HashVec2, rooms: &Vec<Vec<Objects>>) -> bool {
    if pos.y as usize >= rooms.len() || pos.x as usize >= rooms[pos.y as usize].len() {
        return false;
    }

    !matches!(rooms[pos.y as usize][pos.x as usize], Objects::Wall)
}

#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, new)]
pub struct HashVec2 {
    pub x: i32,
    pub y: i32,
}
impl HashVec2 {
    pub fn from_vec2(vec2: Vec2) -> HashVec2 {
        HashVec2 {
            x: vec2.x as i32,
            y: vec2.y as i32,
        }
    }

    pub fn directions(&self, rooms: &Vec<Vec<Objects>>) -> Vec<HashVec2> {
        let mut vec = vec![];
        for pos in [
            HashVec2::new(self.x + 1, self.y),
            HashVec2::new(self.x - 1, self.y),
            HashVec2::new(self.x, self.y + 1),
            HashVec2::new(self.x, self.y - 1),
            HashVec2::new(self.x + 1, self.y + 1),
            HashVec2::new(self.x - 1, self.y + 1),
            HashVec2::new(self.x + 1, self.y - 1),
            HashVec2::new(self.x - 1, self.y - 1),
        ] {
            if pos_valid(&pos, rooms) {
                vec.push(pos);
            }
        }

        vec
    }
}

pub fn astar(start: HashVec2, goal: HashVec2, rooms: &Vec<Vec<Objects>>) -> Option<Vec<HashVec2>> {
    if !pos_valid(&start, rooms) || !pos_valid(&goal, rooms) {
        return None;
    }

    let mut parents = hashmap! {};
    let mut explored = hashset! {};

    let mut pq = PriorityQueue::<HashVec2, i32>::new();
    pq.push(start, 0);

    let mut parent = HashVec2::new(0, 0);
    while let Some(p) = pq.pop() {
        parent = p.0;

        if parent == goal {
            break;
        }
        if explored.contains(&parent) {
            continue;
        }

        explored.insert(parent);

        for child in &parent.directions(rooms) {
            pq.push(*child, manhattan_heuristic(child, &goal));

            if !parents.contains_key(child) {
                parents.insert(*child, parent);
            }
        }
    }

    if parent != goal {
        return None;
    }

    let mut path = vec![goal];
    loop {
        let grandpa = parents.get(&parent);
        if grandpa.is_none() || grandpa.unwrap() == &start {
            break;
        }
        path.insert(0, *grandpa.unwrap());
        parent = *grandpa.unwrap();
    }

    Some(path)
}

#[test]
fn test() {
    let room = vec![
        vec![Objects::Air, Objects::Wall, Objects::Air, Objects::Air],
        vec![Objects::Wall, Objects::Air, Objects::Air, Objects::Air],
        vec![Objects::Air, Objects::Air, Objects::Wall, Objects::Wall],
        vec![Objects::Air, Objects::Air, Objects::Wall, Objects::Air],
    ];
    let goal = HashVec2::new(3, 3);
    let p = HashVec2::new(0, 0);

    println!("{:?}", astar(p, goal, &room));
}
