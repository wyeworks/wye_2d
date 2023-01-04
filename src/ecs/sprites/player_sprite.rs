use ggez::{
    graphics::spritebatch::SpriteBatch,
    mint::{Point2, Vector2},
};
use std::collections::HashMap;
use strum::IntoEnumIterator;

use super::{super::atlas, draw::Draw};
use super::{
    super::systems::{physics_system::physics::*, render_system::camera::Camera},
    sprite::Sprite,
};

pub struct PlayerSprite {
    pub idle_sprites: HashMap<Direction, Sprite>,
    pub walking_sprites: HashMap<Direction, Vec<Sprite>>,
    pub position: Point2<f32>,
}

impl PlayerSprite {
    pub fn new(atlas: &atlas::Atlas) -> Self {
        let mut idle_sprites = HashMap::new();
        let mut walking_sprites = HashMap::new();

        for direction in Direction::iter() {
            idle_sprites.insert(
                direction,
                atlas.create_sprite(
                    &format!("player-idle-{}", Direction::to_index(direction)).to_string(),
                ),
            );
        }

        for direction in Direction::iter() {
            let direction_str: &str = direction.into();
            let mut direction_sprites = Vec::new();

            for animation_frame in 0..=6 {
                direction_sprites.push(atlas.create_sprite(
                    &format!("player-{}-{}", direction_str, animation_frame).to_string(),
                ));
            }

            walking_sprites.insert(direction, direction_sprites);
        }

        Self {
            idle_sprites,
            position: Point2 { x: 500.0, y: 500.0 },
            walking_sprites,
        }
    }
}

impl Draw for PlayerSprite {
    fn draw(
        &mut self,
        batch: &mut SpriteBatch,
        camera: &Camera,
        player_physics: &Physics,
        frames: usize,
    ) {
        let s: &mut Sprite;
        let direction = &player_physics.direction.unwrap();
        if !player_physics.walking {
            s = self.idle_sprites.get_mut(direction).unwrap();
        } else {
            s = &mut self.walking_sprites.get_mut(direction).unwrap()
                [animation_sprite_index(frames)];
        }

        let position = camera.world_to_screen(&player_physics.position);

        batch.add(s.draw_params(
            Point2 {
                x: position.x - s.width,
                y: position.y - s.height,
            },
            Vector2 { x: 2.0, y: 2.0 },
        ));
    }
}

fn animation_sprite_index(frames: usize) -> usize {
    match frames {
        s if s % 70 < 10 => 0,
        s if s % 70 < 20 => 1,
        s if s % 70 < 30 => 2,
        s if s % 70 < 40 => 3,
        s if s % 70 < 50 => 4,
        s if s % 70 < 60 => 5,
        s if s % 70 < 70 => 6,
        _ => 6,
    }
}
