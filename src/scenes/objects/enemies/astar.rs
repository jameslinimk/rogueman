use crate::scenes::{objects::shapes::rect::Rect, rooms::Objects};
use macroquad::prelude::{vec2, Vec2};
use maplit::hashmap;
use priority_queue::PriorityQueue;

fn manhattan_heuristic(pos: &HashVec2, goal: &HashVec2) -> i32 {
    let x_dis = (pos.x - goal.x).abs();
    let y_dis = (pos.y - goal.y).abs();
    return -(x_dis + y_dis);
}

fn pos_valid(pos: &HashVec2, rooms: &Vec<Vec<Objects>>) -> bool {
    if pos.y as usize >= rooms.len() || pos.x as usize >= rooms[pos.y as usize].len() {
        return false;
    }

    match rooms[pos.y as usize][pos.x as usize] {
        Objects::WALL => false,
        _ => true,
    }
}

#[derive(Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
pub struct HashVec2 {
    x: i32,
    y: i32,
}
impl HashVec2 {
    pub fn new(x: i32, y: i32) -> HashVec2 {
        HashVec2 { x, y }
    }

    pub fn from_vec2(vec2: Vec2) -> HashVec2 {
        HashVec2 {
            x: vec2.x as i32,
            y: vec2.y as i32,
        }
    }

    pub fn directions(&self, rooms: &Vec<Vec<Objects>>) -> Vec<HashVec2> {
        let mut vec = vec![];

        let mut add_if_valid = |pos: HashVec2| {
            if pos_valid(&pos, rooms) {
                vec.push(pos);
            }
        };

        for pos in [
            HashVec2::new(self.x + 1, self.y),
            HashVec2::new(self.x - 1, self.y),
            HashVec2::new(self.x, self.y + 1),
            HashVec2::new(self.x, self.y - 1),
        ] {
            if pos_valid(&pos, rooms) {
                vec.push(pos);
            }
        }

        vec
    }
}

pub fn astar(start: HashVec2, goal: HashVec2, rooms: &Vec<Vec<Objects>>) {
    let mut parents = hashmap! {};

    let mut pq = PriorityQueue::<HashVec2, i32>::new();
    pq.push(start, 0);

    let mut parent = HashVec2::new(0, 0);
    loop {
        let p = pq.pop();
        if p.is_none() {
            // return None;
            return;
        }
        parent = p.unwrap().0;

        if parent == goal {
            break;
        }

        for child in &parent.directions(rooms) {
            pq.push(*child, manhattan_heuristic(child, &goal));
            parents.insert(parent, *child);
        }
    }

    // TODO TEST

    let mut path = vec![];
    loop {
        let grandpa = parents.get(&parent);
        if grandpa.is_none() {
            break;
        }
        path.push(grandpa.unwrap());
    }

    println!("path: {:?}", path);
}

fn print_rooms(rooms: &Vec<Vec<Objects>>) {
    for row in rooms {
        for cell in row {
            print!("{:?} ", cell);
        }
        println!();
    }
}

#[test]
fn test() {
    let rooms = vec![vec![Objects::AIR; 8]; 8];
    let goal = HashVec2::new(5, 5);
    let p = HashVec2::new(1, 0);

    astar(p, goal, &rooms);
}
