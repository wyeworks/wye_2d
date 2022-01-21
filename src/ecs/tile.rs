use ggez::{graphics::spritebatch::SpriteBatch, mint::Point2};

use super::{
    atlas::{self, Sprite},
    systems::{camera_system::Camera, physics_system::positioning::positioning::Position},
};
pub const NUMBER_OF_TILES: u8 = 3;
pub const INTIAL_WORLD_W: f32 = 2000.0;
pub const INTIAL_WORLD_H: f32 = 1000.0;
pub struct TileEntity {
    pub sprite: Sprite,
    pub position: Point2<f32>,
}

impl TileEntity {
    pub fn draw(&mut self, batch: &mut SpriteBatch, camera: &Camera) {
        let s = &mut self.sprite;
        let position = camera.world_to_screen(&Position {
            x: self.position.x,
            y: self.position.y,
        });

        batch.add(s.add_draw_param(Point2 {
            x: position.x,
            y: position.y,
        }));
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

fn create_tile_scroll(sprite: Sprite, x: f32, y: f32) -> Box<TileEntity> {
    //let position = camera.world_to_screen(&Position { x, y: 300.0 });
    let tile = TileEntity::new(sprite, (x, y /*position.x, position.y*/));
    // floor tiles do not need to move... do they ?!
    // let tile = tile.scroller(jump).set_velocity((-1.0, 0.0));

    Box::new(tile)
}

pub fn create_tiles(sprites: &atlas::Atlas) -> Vec<Box<TileEntity>> {
    let floor_tile = sprites.create_sprite("floor_tile.png");
    let width = floor_tile.width;
    let height = floor_tile.height;

    let mut tiles: Vec<Box<TileEntity>> = Vec::new();

    for y in (0..INTIAL_WORLD_H as i32).step_by(height as usize) {
        let mut tile_row: Vec<Box<TileEntity>> = (0..INTIAL_WORLD_W as i32)
            .step_by(width as usize)
            .into_iter()
            .map(|i| {
                create_tile_scroll(
                    floor_tile.clone(),
                    i as f32,
                    y as f32,
                )
            })
            .collect();

        tiles.append(&mut tile_row);
    }
    tiles
}
