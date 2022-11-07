use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use macroquad::texture::{load_texture, Texture2D};
use maplit::hashmap;

lazy_static! {
    static ref ASSET_MAP: Mutex<HashMap<&'static str, Texture2D>> = Mutex::new(hashmap!());
}

pub fn get_image(path: &str) -> Option<Texture2D> {
    match ASSET_MAP.lock().unwrap().get(path) {
        Some(texture) => Option::from(texture.to_owned()),
        None => None,
    }
}

pub async fn load_image(path: &'static str) -> Texture2D {
    if ASSET_MAP.lock().unwrap().contains_key(path) {
        return get_image(path).unwrap();
    }
    let resource = load_texture(path).await.unwrap();
    ASSET_MAP.lock().unwrap().insert(path, resource.to_owned());
    resource
}
