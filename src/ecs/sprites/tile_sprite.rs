use crate::ecs::{
    atlas::Atlas,
    sprites::sprite::Sprite,
    systems::{physics_system::physics::Position, render_system::camera::Camera},
    utils::constants::{INTIAL_WORLD_H, INTIAL_WORLD_W},
};

use ggez::{
    graphics::spritebatch::SpriteBatch,
    mint::{Point2, Vector2},
};

pub const NUMBER_OF_TILES: u8 = 3;
pub struct TileSprite {
    pub sprite: Sprite,
    pub position: Point2<f32>,
}

impl TileSprite {
    pub fn draw(&mut self, batch: &mut SpriteBatch, camera: &Camera) {
        let s = &mut self.sprite;
        let position = camera.world_to_screen(&Position {
            x: self.position.x,
            y: self.position.y,
        });

        batch.add(s.draw_params(
            Point2 {
                x: position.x,
                y: position.y,
            },
            Vector2 { x: 1.0, y: 1.0 },
        ));
    }

    pub fn new(sprite: Sprite, position: (f32, f32)) -> Self {
        Self {
            sprite,
            position: Point2 {
                x: position.0,
                y: position.1,
            },
        }
    }
}

fn create_tile(sprite: Sprite, x: f32, y: f32) -> Box<TileSprite> {
    let tile = TileSprite::new(sprite, (x, y));

    Box::new(tile)
}

pub fn create_tiles(sprites: &Atlas) -> Vec<Box<TileSprite>> {
    let floor_tile = sprites.create_sprite("floor_tile.png");
    let width = floor_tile.width;
    let height = floor_tile.height;

    let mut tiles: Vec<Box<TileSprite>> = Vec::new();

    for y in (0..INTIAL_WORLD_H as i32).step_by(height as usize) {
        let mut tile_row: Vec<Box<TileSprite>> = (0..INTIAL_WORLD_W as i32)
            .step_by(width as usize)
            .into_iter()
            .map(|i| create_tile(floor_tile.clone(), i as f32, y as f32))
            .collect();

        tiles.append(&mut tile_row);
    }
    tiles
}
