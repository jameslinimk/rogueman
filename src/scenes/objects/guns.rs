use super::bullet::BulletConfig;

#[derive(Debug)]
enum Rarities {
    COMMON,
    UNCOMMON,
    RARE,
    EPIC,
    LEGENDARY,
    UNIQUE,
}

#[derive(Debug)]
pub(crate) struct Gun {
    name: String,
    holdable: bool,
    fire_delay: f32,
    rarity: Rarities,
    level: u8,
    bullet_config: BulletConfig,
}

pub(crate) fn pistol() -> Gun {
    return Gun {
        name: "Pistol".to_string(),
        holdable: false,
        fire_delay: 0.2,
        rarity: Rarities::COMMON,
        level: 1,
        bullet_config: BulletConfig {
            speed: 1000.0,
            max_lifespan: 5.0,
            spread: 15.0,
            bullet_size: 15.0,
            pierce: 0,
            damage: 10.0,
            friendly: true,
        },
    };
}
