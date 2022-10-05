use std::collections::HashMap;

use macroquad::texture::{load_texture, Texture2D};

static mut ASSET_MAP: Option<HashMap<&str, Texture2D>> = None;

pub(crate) fn get_image(path: &str) -> Option<Texture2D> {
    unsafe {
        if ASSET_MAP.is_none() {
            ASSET_MAP = Option::from(HashMap::new());
        }

        match ASSET_MAP.as_ref().unwrap().get(path) {
            Some(texture) => {
                return Option::from(texture.to_owned());
            }
            None => return None,
        };
    }
}

pub(crate) async fn load_image(path: &str) -> Texture2D {
    unsafe {
        if ASSET_MAP.is_none() {
            ASSET_MAP = Option::from(HashMap::new());
        }

        let resource = load_texture(path).await.unwrap();
        ASSET_MAP
            .as_mut()
            .unwrap()
            .insert(path, resource.to_owned());
        return resource;
    }
}
