use ggez::{
    graphics::spritebatch::SpriteBatch,
    mint::{Point2, Vector2},
};

use super::{
    super::systems::{
        physics_system::positioning::positioning::*, render_system::camera_system::Camera,
    },
    atlas::{self, Sprite},
};

pub struct PlayerSprite {
    pub idle_sprites: Vec<Sprite>,
    pub walking_sprites: Vec<Vec<Sprite>>,
    pub position: Point2<f32>,
}

impl PlayerSprite {
    pub fn draw(
        &mut self,
        batch: &mut SpriteBatch,
        camera: &Camera,
        player_physics: &Physics,
        frames: usize,
    ) {
        let s: &mut Sprite;
        if !player_physics.walking {
            s = &mut self.idle_sprites[index_from_direction(player_physics.direction.unwrap())];
            /*
            s = match player_physics.direction.unwrap() {
                Direction::Up => &mut self.idle_sprites[0],
                Direction::Right => &mut self.idle_sprites[1],
                Direction::Down => &mut self.idle_sprites[2],
                Direction::Left => &mut self.idle_sprites[3],
            };
            */
        } else {
            s = match player_physics.direction.unwrap() {
                Direction::Up => {
                    &mut self.walking_sprites[index_from_direction(Direction::Up)]
                        [animation_sprite_index(frames)]
                }
                Direction::Right => {
                    &mut self.walking_sprites[index_from_direction(Direction::Right)]
                        [animation_sprite_index(frames)]
                }
                Direction::Down => {
                    &mut self.walking_sprites[index_from_direction(Direction::Down)]
                        [animation_sprite_index(frames)]
                }
                Direction::Left => {
                    &mut self.walking_sprites[index_from_direction(Direction::Left)]
                        [animation_sprite_index(frames)]
                }
            }
        }

        let position = camera.world_to_screen(&player_physics.position);

        batch.add(s.draw_params(
            Point2 {
                x: position.x - s.width,  // * 2.0 / 2.0,
                y: position.y - s.height, // * 2.0 / 2.0,
            },
            Vector2 { x: 2.0, y: 2.0 },
        ));
    }

    pub fn new(
        idle_sprites: Vec<Sprite>,
        walking_sprites: Vec<Vec<Sprite>>,
        position: (f32, f32),
    ) -> Self {
        Self {
            idle_sprites,
            position: Point2 {
                x: position.0,
                y: position.1,
            },
            walking_sprites,
        }
    }
}

pub fn create_player_sprite(atlas: &atlas::Atlas) -> PlayerSprite {
    let mut player_sprites = Vec::new();
    let mut walking_sprites = Vec::new();

    for direction in 0..=3 {
        player_sprites.push(atlas.create_sprite(&format!("player-idle-{}", direction).to_string()));
    }
    for direction in 0..=3 {
        let direction_name = direction_from_index(direction);
        let mut direction_sprites = Vec::new();
        for animation_frame in 0..=6 {
            direction_sprites.push(atlas.create_sprite(
                &format!("player-{}-{}", direction_name, animation_frame).to_string(),
            ));
        }
        walking_sprites.push(direction_sprites);
    }
    PlayerSprite::new(
        player_sprites,
        walking_sprites,
        (500.0, 500.0 /*position.x, position.y*/),
    )
}

fn index_from_direction(direction: Direction) -> usize {
    match direction {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Down => 2,
        Direction::Left => 3,
    }
}

fn direction_from_index(index: usize) -> String {
    match index {
        0 => "up".to_string(),
        1 => "right".to_string(),
        2 => "down".to_string(),
        3 => "left".to_string(),
        _ => "right".to_string(),
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
