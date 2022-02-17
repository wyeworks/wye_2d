use super::positioning::{collision::objects_collide, positioning::*};
use crate::ecs::{constants::*, game_state::EntityIndex, systems::camera_system::Camera};
use ggez::{event::KeyCode, graphics, Context, GameResult};
use rand::Rng;

pub enum Entity {
    Player,
    Npc,
}

pub fn generate_physics(entity_type: Entity) -> Physics {
    let position: Position;
    let size: Size;
    let speed: f32;
    let color: graphics::Color;
    let direction: Option<Direction>;

    match entity_type {
        Entity::Player => {
            position = INITIAL_PLAYER_POS;
            size = Size {
                width: HUMANOID_W,
                height: HUMANOID_H,
            };
            speed = INITIAL_PLAYER_SPEED;
            color = graphics::Color::from_rgb(0, 171, 169);
            direction = Some(Direction::Down);
        }
        Entity::Npc => {
            position = Position::from_f32(get_random_position());
            size = Size {
                width: HUMANOID_W,
                height: HUMANOID_H,
            };
            speed = 0.0;
            color = graphics::Color::from_rgb(112, 111, 211);
            direction = None;
        }
    }

    Physics {
        position,
        size,
        speed,
        color,
        direction,
        walking: false,
    }
}

pub fn get_random_position() -> (f32, f32) {
    let x = rand::thread_rng().gen_range(300..900);
    let y = rand::thread_rng().gen_range(200..700);
    (x as f32, y as f32)
}

pub fn update_player_physics(
    ctx: &mut Context,
    physics_components: &Vec<Option<Physics>>,
    current_focus: &mut Option<EntityIndex>,
    player_physics: &mut Physics,
    player_mov_actions: &Vec<KeyCode>,
    camera: &mut Camera,
    world_size: &Size,
) -> GameResult {
    if player_mov_actions.len() == 0 {
        player_physics.walking = false;
    }
    for key in player_mov_actions.iter() {
        let mut new_potential_player_physics = player_physics.clone();
        new_potential_player_physics.update_position(ctx, *key, world_size);

        let should_update_camera = camera
            .is_player_approaching_camera_edge(&new_potential_player_physics.position, *key)
            && camera.is_within_world_bounds(world_size, *key);

        if should_update_camera {
            camera.update_position(*key, ctx);
        }

        let mut player_collides: bool = false;

        for (index, object_physics) in physics_components.iter().enumerate() {
            match object_physics {
                Some(physics) => {
                    if objects_collide(&new_potential_player_physics, &physics) {
                        player_collides = true;
                        *current_focus = Some(index);
                    };
                }
                None => continue,
            }
        }

        let last_mov_key = player_mov_actions.last().unwrap();

        new_potential_player_physics.direction = match last_mov_key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => player_physics.direction,
        };

        if !player_collides {
            *current_focus = None;
            *player_physics = new_potential_player_physics;
        }
    }
    Ok(())
}
