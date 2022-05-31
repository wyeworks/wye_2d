use super::physics::*;
use crate::ecs::{utils::constants::*};
use ggez::{event::KeyCode, graphics, Context};
use rand::Rng;

// Physics generators
pub fn initial_player_physics() -> Physics {
    Physics {
        position: INITIAL_PLAYER_POS,
        size: Size {
            width: HUMANOID_W,
            height: HUMANOID_H,
        },
        speed: INITIAL_PLAYER_SPEED,
        color: graphics::Color::from_rgb(0, 171, 169),
        direction: Some(Direction::Down),
        walking: false,
        current_focus: None
    }
}

pub fn generate_npc_physics() -> Physics {
    Physics {
        position: Position::from_f32(get_random_position()),
        size: Size {
            width: HUMANOID_W,
            height: HUMANOID_H,
        },
        speed: 0.0,
        color: graphics::Color::from_rgb(112, 111, 211),
        direction: None,
        walking: false,
        current_focus: None
    }
}

fn get_random_position() -> (f32, f32) {
    let x = rand::thread_rng().gen_range(300..900);
    let y = rand::thread_rng().gen_range(200..700);
    (x as f32, y as f32)
}

// Update physics
pub fn update_player_physics(
    ctx: &mut Context,
    player_mov_actions: &Vec<KeyCode>,
    player_physics: &Physics,
    physics_components: &Vec<Option<Physics>>,
    world_size: &Size,
) -> Physics {
    let mut new_player_physics = player_physics.clone();

    if player_mov_actions.len() == 0 {
        new_player_physics.walking = false;
    }

    for key in player_mov_actions.iter() {
        let mut new_potential_player_physics = new_player_physics.clone();
        new_potential_player_physics.update_position(ctx, *key, &world_size);

        let mut player_collides: bool = false;

        for (index, object_physics) in physics_components.iter().enumerate() {
            match object_physics {
                Some(physics) => {
                    if objects_collide(&new_potential_player_physics, &physics) {
                        player_collides = true;
                        new_potential_player_physics.current_focus = Some(index);
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
            _ => new_player_physics.direction,
        };

        if !player_collides {
            new_potential_player_physics.current_focus = None;
            new_player_physics = new_potential_player_physics;
        }
    }
    
    new_player_physics
}

fn objects_collide(a: &Physics, b: &Physics) -> bool {
    let collision = a.position.x - a.size.w_half() < b.position.x + b.size.w_half()
        && a.position.x + a.size.w_half() > b.position.x - b.size.w_half()
        && a.position.y - a.size.h_half() < b.position.y + b.size.h_half()
        && a.size.h_half() + a.position.y > b.position.y - b.size.h_half();
    return collision;
}

// TO DO
// - Physics system logic
// - Render system
// - Camera system
