use crate::positioning::{collision::objects_collide, positioning::*};

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

        for n in 2..6 {
            initial_desks.push(
                Desk::new(Position {
                x: 200.0,
                y: n as f32 * 120.0
            }));
        }
        Map {
            player: Player::new(),
            desks: initial_desks,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let player_mov_keys = [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right];

        for key in player_mov_keys {
            if keyboard::is_key_pressed(ctx, key) {
                let mut new_potential_player_body = self.player.body.clone();

                new_potential_player_body.update_position(key, ctx);
                let mut player_collides: bool = false;
                for desk in self.desks.iter() {
                    if objects_collide(&new_potential_player_body, &desk.body) {
                        player_collides = true;
                    }
                }
                if !player_collides {
                    self.player.body.position = new_potential_player_body.position;
                }
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
