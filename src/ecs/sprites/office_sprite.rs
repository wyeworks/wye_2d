use ggez::{
    graphics::spritebatch::SpriteBatch,
    mint::{Point2, Vector2},
};
use std::collections::HashMap;

use crate::ecs::components::desk::Desk;

use super::{super::atlas, draw::DrawComponent};
use super::{
    super::systems::{physics_system::physics::*, render_system::camera::Camera},
    sprite::Sprite,
};

const FRAME_INTERVAL_DESK_ANIMATION: i32 = 120;

pub struct OfficeSprite {
    pub desk_sprites: HashMap<i32, Vec<Sprite>>,
    pub position: Point2<f32>,
}

impl OfficeSprite {
    pub fn new(atlas: &atlas::Atlas) -> Self {
        let mut desk_sprites = HashMap::new();

        desk_sprites.insert(
            0,
            vec!(
                atlas.create_sprite(&format!("desk-type-a-0")),
                atlas.create_sprite(&format!("desk-type-a-1")),
            ) 
        );

        desk_sprites.insert(
            1,
            vec!(
                atlas.create_sprite(&format!("desk-type-b-0")),
                atlas.create_sprite(&format!("desk-type-b-1")),
            ) 
        );

        Self {
            desk_sprites,
            position: Point2 { x: 0.0, y: 0.0 },
        }
    }
}

impl DrawComponent for OfficeSprite {
    type Component = Desk;

    fn draw_component(
        &mut self,
        batch: &mut SpriteBatch,
        camera: &Camera,
        physics: &Physics,
        frames: usize,
        component: &Desk
    ) {
        let sprite = &self.desk_sprites.get_mut(&component.desk_type).unwrap()
            [animation_sprite_index(frames, component.animation_id)];

        let position = camera.world_to_screen(&physics.position);

        batch.add(sprite.draw_params(
            Point2 {
                x: position.x - sprite.width,
                y: position.y - sprite.height,
            },
            Vector2 { x: 2.0, y: 2.0 },
        ));
    }
}

fn animation_sprite_index(frames: usize, animation_id: i32) -> usize {
    let modifier = animation_id * 10;
    if frames as i32 % (FRAME_INTERVAL_DESK_ANIMATION * 2) < FRAME_INTERVAL_DESK_ANIMATION + modifier  {
        0
    } else {
        1
    }
}