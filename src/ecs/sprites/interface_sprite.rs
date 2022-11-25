use ggez::{
    graphics::spritebatch::SpriteBatch,
    mint::{Point2, Vector2},
};
use super::super::atlas;
use super::sprite::Sprite;

pub struct InterfaceSprite  {
    life_outline_sprite: Sprite,
    life_filled_sprite: Sprite

}

impl InterfaceSprite {
    pub fn new(atlas: &atlas::Atlas) -> Self {       
        Self {
            life_outline_sprite: atlas.create_sprite(&"heart-0".to_string()),
            life_filled_sprite: atlas.create_sprite(&"heart-1".to_string()),
        }
    }
    pub fn draw(
        &mut self,
        batch: &mut SpriteBatch,
    ) {
        // let s: &mut Sprite = &mut self.sprites[npc.id as usize];
        // let position = camera.world_to_screen(&physics.position);

        let sprite: &mut Sprite = &mut self.life_filled_sprite;

        batch.add(sprite.draw_params(
            Point2 {
                x: 600.0,
                y: 600.0,
            },
            Vector2 { x: 2.0, y: 2.0 },
        ));
    }
}
