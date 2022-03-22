use ggez::*;
use ggez::{event::KeyCode, input::keyboard};

pub fn handle_input(ctx: &mut Context) -> Vec<KeyCode> {
    let player_mov_keys = [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right];
    let mut pressed_mov_keys: Vec<KeyCode> = Vec::new();

    for key in player_mov_keys {
        if keyboard::is_key_pressed(ctx, key) {
            pressed_mov_keys.push(key);
        }
    }
    pressed_mov_keys
}
