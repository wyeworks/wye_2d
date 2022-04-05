use ggez::*;
use ggez::{event::KeyCode, input::keyboard};

use crate::ecs::{
    game_state::GameState, systems::physics_system::positioning::collision::Interaction,
};

pub fn player_movements(ctx: &mut Context) -> Vec<KeyCode> {
    let player_mov_keys = [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right];
    let mut pressed_mov_keys: Vec<KeyCode> = Vec::new();

    for key in player_mov_keys {
        if keyboard::is_key_pressed(ctx, key) {
            pressed_mov_keys.push(key);
        }
    }
    pressed_mov_keys
}

// Interactions
pub fn key_down_event_interaction(game_state: &GameState, key: KeyCode) -> Option<Interaction> {
    match game_state.current_interaction {
        Some(_) => interaction_input_handler(game_state, key),
        None => match key {
            KeyCode::Return => begin_interaction(game_state),
            _ => None,
        },
    }
}

fn interaction_input_handler(game_state: &GameState, key: KeyCode) -> Option<Interaction> {
    match key {
        KeyCode::Up | KeyCode::Down | KeyCode::Return => update_interaction(game_state, key),
        KeyCode::Escape => None,
        _ => None,
    }
}

fn begin_interaction(game_state: &GameState) -> Option<Interaction> {
    match game_state.current_focus {
        Some(focused_entity_id) => match &game_state.npcs_components[focused_entity_id] {
            Some(_) => game_state.npcs_interactions[focused_entity_id].clone(),
            None => None,
        },
        None => None,
    }
}

fn update_interaction(game_state: &GameState, action: KeyCode) -> Option<Interaction> {
    let mut interaction = game_state.current_interaction.clone().unwrap();
    match &interaction.sub_interactions {
        Some(sub_interactions) => match action {
            KeyCode::Up => {
                if interaction.hovered_option != 0 {
                    interaction.hovered_option -= 1;
                }
                Some(interaction)
            }
            KeyCode::Down => {
                if interaction.hovered_option < interaction.options.as_ref().unwrap().len() - 1 {
                    interaction.hovered_option += 1;
                }
                Some(interaction)
            }
            KeyCode::Return => Some(sub_interactions[interaction.hovered_option].clone()),
            _ => None,
        },
        None => None,
    }
}
