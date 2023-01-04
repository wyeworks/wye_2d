use crate::ecs::{
    atlas::Atlas,
    components::npc::Npc,
    sprites::sprite::Sprite,
    systems::{physics_system::physics::*, render_system::camera::Camera},
    utils::constants::NPC_COUNT,
};

use ggez::{
    graphics::spritebatch::SpriteBatch,
    mint::{Point2, Vector2},
};

pub struct NpcSprite {
    pub sprites: Vec<Sprite>,
}

impl NpcSprite {
    pub fn new(atlas: &Atlas) -> Self {
        let mut sprites = Vec::new();

        for npc_count in 0..NPC_COUNT {
            sprites.push(atlas.create_sprite(&format!("npc_{}", npc_count).to_string()));
        }

        Self { sprites }
    }

    pub fn draw(&mut self, batch: &mut SpriteBatch, camera: &Camera, physics: &Physics, npc: &Npc) {
        let s: &mut Sprite = &mut self.sprites[npc.id as usize];
        let position = camera.world_to_screen(&physics.position);

        batch.add(s.draw_params(
            Point2 {
                x: position.x - s.width,
                y: position.y - s.height,
            },
            Vector2 { x: 2.0, y: 2.0 },
        ));
    }
}
