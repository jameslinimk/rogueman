use super::items::Rarities;

#[derive(Debug, Clone, Copy)]
pub struct Melee {
    pub name: &'static str,
    pub image_file: &'static str,
    pub damage: f32,
    pub range: f32,
    pub range_width: f32,
    pub delay: f32,
    pub swing_duration: f32,
    pub rarity: Rarities,
}

const POCKET_KNIFE: Melee = Melee {
    name: "Pocket Knife",
    image_file: "./assets/melees/pocket_knife.png",
    damage: 25.0,
    range: 25.0,
    range_width: 10.0,
    delay: 0.5,
    swing_duration: 1.0,
    rarity: Rarities::COMMON,
};

const POCKET_KNIFE_2: Melee = Melee {
    name: "Pocket Knife 2",
    image_file: "./assets/melees/pocket_knife.png",
    damage: 25.0,
    range: 25.0,
    range_width: 10.0,
    delay: 1.0,
    swing_duration: 1.0,
    rarity: Rarities::COMMON,
};

pub const MELEES: [Melee; 2] = [POCKET_KNIFE, POCKET_KNIFE_2];
