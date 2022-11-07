use derive_new::new;
use macroquad::prelude::{Rect, Texture2D, WHITE};
use macroquad::text::draw_text_ex;
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::time::{get_frame_time, get_time};

#[derive(new)]
pub struct SpriteSheet {
    base_texture: Texture2D,
    width: u16,
    frame_duration: f32,

    #[new(value = "f64::MIN")]
    last_frame: f64,
    #[new(value = "0")]
    current_frame: u16,
}
impl SpriteSheet {
    pub fn draw(&self, x: f32, y: f32) {
        let width = self.base_texture.width();
        let height = self.base_texture.height();

        let rect = Rect::new(self.current_frame as f32 * width, 0.0, width, height);
        draw_texture_ex(
            self.base_texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Option::from(rect),
                ..DrawTextureParams::default()
            },
        )
    }

    pub fn update(&mut self) {
        if get_time() > self.last_frame + self.frame_duration as f64 {
            self.current_frame += 1;
            if self.current_frame >= self.width {
                self.current_frame = 0;
            }
            self.last_frame = get_time();
        }
    }
}
