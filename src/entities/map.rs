use crate::Position;

use super::desk::*;
use super::player::*;
use ggez::*;
use ggez::{event::*, input::keyboard};
pub struct Map {
    player: Player,
    desks: Vec<Desk>,
}

impl Map {
    // Creates a new map, with the inital positions of its objects (could be on each objects' ::new)
    pub fn new() -> Self {
        let mut initial_desks = Vec::new();
        let first_desk = Desk {
            position: Position { x: 200.0, y: 200.0 },
        };
        initial_desks.push(first_desk);

        Map {
            player: Player::new(),
            desks: initial_desks,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let player_mov_keys = [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right];
        for key in player_mov_keys {
            if keyboard::is_key_pressed(ctx, key) {
                self.player.walk(key, ctx);
            }
        }

        Ok(())
    }
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(130, 90, 44));

        // We iterate over each desk in the map and tell them to draw themselves
        for desk in self.desks.iter_mut() {
            desk.draw(ctx)?;
        }

        self.player.draw(ctx)?;

        Ok(())
    }
}
