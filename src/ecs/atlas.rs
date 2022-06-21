use ggez::{Context, graphics::{self, spritebatch::SpriteBatch}};
use std::path::Path;

use super::sprites::sprite::Sprite;

#[derive(Deserialize, Debug)]
struct Meta {
    size: AtlasSize,
}

#[derive(Deserialize, Debug)]
struct AtlasSize {
    w: i32,
    h: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct JsonRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct SpriteData {
    filename: String,
    frame: JsonRect,
}

#[derive(Deserialize, Debug)]
pub struct Atlas {
    frames: Vec<SpriteData>,
    meta: Meta,
}

impl Atlas {
    pub fn parse_atlas_json(texture_atlas_file: &Path) -> Self {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(texture_atlas_file).expect("Couldn't find the texture_atlas file");
        let buf_reader = BufReader::new(file);
        serde_json::from_reader(buf_reader).expect("Couldn't create texture atlas")
    }

    /// Returns a sprite from the Atlas.
    pub fn create_sprite(&self, sprite_name: &str) -> Sprite {
        let width = self.meta.size.w as f32;
        let height = self.meta.size.h as f32;
        let atlas_rect = graphics::Rect::new(0.0, 0.0, width, height);

        if let Some(sprite_data) = self.frames.iter().find(|d| d.filename == sprite_name) {
            Sprite::new(
                graphics::Rect::fraction(
                    sprite_data.frame.x as f32,
                    sprite_data.frame.y as f32,
                    sprite_data.frame.w as f32,
                    sprite_data.frame.h as f32,
                    &atlas_rect,
                ),
                sprite_data.frame.w as f32,
                sprite_data.frame.h as f32,
            )
        } else {
            unimplemented!("Not handling failure to find sprite");
        }
    }
}

pub fn create_batch_sprite(ctx: &mut Context, file_name: String) -> SpriteBatch {
    let image = graphics::Image::new(ctx, file_name).unwrap();
    let mut batch = graphics::spritebatch::SpriteBatch::new(image);
    batch.set_filter(graphics::FilterMode::Nearest);
    batch
}
