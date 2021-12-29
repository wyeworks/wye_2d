use super::positioning::{collision::objects_collide, positioning::*};
use crate::ecs::{constants::*, game_state::EntityIndex};
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

    match entity_type {
        Entity::Player => {
            position = INITIAL_PLAYER_POS;
            size = Size {
                width: HUMANOID_W,
                height: HUMANOID_H,
            };
            speed = INITIAL_PLAYER_SPEED;
            color = graphics::Color::from_rgb(0, 171, 169);
        }
        Entity::Npc => {
            position = Position::from_f32(get_random_position());
            size = Size {
                width: HUMANOID_W,
                height: HUMANOID_H,
            };
            speed = 0.0;
            color = graphics::Color::from_rgb(112, 111, 211);
        }
    }

    Physics {
        position,
        size,
        speed,
        color,
    }
}

pub fn get_random_position() -> (f32, f32) {
    let x = rand::thread_rng().gen_range(300..900);
    let y = rand::thread_rng().gen_range(200..700);
    (x as f32, y as f32)
}

pub fn update_player_physics(
    physics_components: &Vec<Option<Physics>>,
    current_focus: &mut Option<EntityIndex>,
    player_physics: &mut Physics,
    player_mov_actions: &Vec<KeyCode>,
    ctx: &mut Context,
) -> GameResult {
    for key in player_mov_actions.iter() {
        let mut new_potential_player_physics = player_physics.clone();
        new_potential_player_physics.update_position(*key, ctx);
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

        if !player_collides {
            *player_physics = new_potential_player_physics;
        }
    }
    Ok(())
}
