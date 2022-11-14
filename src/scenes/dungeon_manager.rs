use super::room_gen::gen::Objects;

#[derive(Clone)]
pub struct Manager {
    pub room: Vec<Vec<Objects>>,
    pub doors: Vec<(usize, usize)>,
}
