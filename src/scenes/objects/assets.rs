use std::{collections::HashMap, hash::Hash, sync::Mutex};

use lazy_static::lazy_static;
use macroquad::texture::{load_texture, Texture2D};

lazy_static! {
    static ref ASSET_MAP: Mutex<HashMap<&'static str, Texture2D>> = Mutex::new(HashMap::new());
}

pub(crate) fn get_image(path: &str) -> Option<Texture2D> {
    match ASSET_MAP.lock().unwrap().get(path) {
        Some(texture) => {
            return Option::from(texture.to_owned());
        }
        None => return None,
    };
}

pub(crate) async fn load_image(path: &str) -> Texture2D {
    let resource = load_texture(path).await.unwrap();
    ASSET_MAP.lock().unwrap().insert(path, resource.to_owned());
    return resource;
}
