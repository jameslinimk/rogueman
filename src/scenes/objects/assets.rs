use std::collections::HashMap;

use macroquad::texture::Texture2D;

static mut ASSET_MAP: Option<HashMap<&str, Texture2D>> = None;

pub async fn(crate) load_asset(path: &str) {
	if ASSET_MAP.is_none() {
		
	}

}
