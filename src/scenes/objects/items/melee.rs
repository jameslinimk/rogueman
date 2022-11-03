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
    range: 70.0,
    range_width: 50.0,
    delay: 0.2,
    swing_duration: 0.1,
    rarity: Rarities::COMMON,
};

pub const MELEES: [Melee; 1] = [POCKET_KNIFE];
