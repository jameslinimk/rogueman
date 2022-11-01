use super::items::Rarities;

#[derive(Debug, Clone, Copy)]
pub struct Melee {
    pub name: &'static str,
    pub image_file: &'static str,
    damage: f32,
    range: f32,
    range_width: f32,
    pub delay: f32,
    rarity: Rarities,
}

const POCKET_KNIFE: Melee = Melee {
    name: "Pocket Knife",
    image_file: "./assets/melees/pocket_knife.png",
    damage: 25.0,
    range: 25.0,
    range_width: 10.0,
    delay: 1.0,
    rarity: Rarities::COMMON,
};

pub const MELEES: [Melee; 1] = [POCKET_KNIFE];
