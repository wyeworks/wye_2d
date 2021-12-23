use super::positioning::{collision::objects_collide, positioning::*};
use crate::ecs::{
    components::{npc::Npc, player::Player},
    constants::*,
};
use ggez::{event::KeyCode, graphics, input::keyboard, Context, GameResult};
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

pub fn update_physics(
    physics_components: &mut Vec<Option<Physics>>,
    npcs_components: &mut Vec<Option<Npc>>,
    player_components: &mut Vec<Option<Player>>,
    ctx: &mut Context,
) -> GameResult {
    let player_mov_keys = [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right];

    for key in player_mov_keys {
        if keyboard::is_key_pressed(ctx, key) {
            let mut new_potential_player_physics = physics_components[0].unwrap().clone();

            new_potential_player_physics.update_position(key, ctx);
            let mut player_collides: bool = false;

            for (index, object_physics) in physics_components.iter().enumerate() {
                match object_physics {
                    Some(physics) => {
                        if index == 0 {
                            continue;
                        };
                        if objects_collide(&new_potential_player_physics, &physics) {
                            player_collides = true;
                            let npc_index = npcs_components.get(index);
                            match npc_index {
                                Some(npc) => match npc {
                                    Some(_) => {
                                        let mut player_ref = player_components[0].unwrap(); //.clone();
                                        player_ref.current_focus = Some(index);
                                        player_components[0] = Some(player_ref);
                                    }
                                    None => (),
                                },
                                None => continue,
                            }
                        }
                    }
                    None => continue,
                }
            }

            if !player_collides {
                physics_components[0] = Some(new_potential_player_physics);
            }
        }
    }
    Ok(())
}
