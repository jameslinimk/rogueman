use super::item::Rarities;
use crate::camera::ShakeConfig;
use crate::scenes::objects::bullet::BulletConfig;

#[derive(Debug, Clone, Copy)]
pub struct Gun {
    pub name: &'static str,
    pub image_file: &'static str,
    pub holdable: bool,
    pub fire_delay: f32,
    pub rarity: Rarities,
    pub level: u8,
    pub bullet_config: BulletConfig,
    pub shake: ShakeConfig,
}

const PISTOL: Gun = Gun {
    name: "Pistol",
    image_file: "./assets/guns/pistol.png",
    holdable: true,
    fire_delay: 0.2,
    rarity: Rarities::Common,
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
    shake: ShakeConfig {
        duration: 0.5,
        intensity: 70.0,
    },
};

const SMG: Gun = Gun {
    name: "SMG",
    image_file: "./assets/guns/smg.png",
    holdable: true,
    fire_delay: 0.1,
    rarity: Rarities::Common,
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
    shake: ShakeConfig {
        duration: 0.5,
        intensity: 70.0,
    },
};

pub const GUNS: [Gun; 2] = [PISTOL, SMG];
