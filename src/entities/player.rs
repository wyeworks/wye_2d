use ggez::event::*;
use ggez::graphics::*;
use ggez::*;

use crate::{clamp, Position};

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_W: f32 = 25.0;
const PLAYER_H: f32 = 25.0;
const PLAYER_W_HALF: f32 = PLAYER_W * 0.5;
const PLAYER_H_HALF: f32 = PLAYER_H * 0.5;

pub struct Player {
    pub position: Position,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Position { x: 200.0, y: 300.0 },
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(
            self.position.x - PLAYER_W_HALF,
            self.position.y - PLAYER_H_HALF,
            PLAYER_W,
            PLAYER_H,
        );

        let rect_mesh = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            rect,
            graphics::Color::from_rgb(0, 171, 169),
        )?;

        graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

        Ok(())
    }

    pub fn walk(&mut self, direction: KeyCode, ctx: &mut Context) {
        // We use deltatime to say: I want to move this object 'speed' meters per second. So our code is not dependent on frame rates.
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        match direction {
            KeyCode::Up => self.position.y -= PLAYER_SPEED * dt,
            KeyCode::Down => self.position.y += PLAYER_SPEED * dt,
            KeyCode::Left => self.position.x -= PLAYER_SPEED * dt,
            KeyCode::Right => self.position.x += PLAYER_SPEED * dt,
            _ => (),
        }

        let (screen_w, screen_h) = graphics::drawable_size(ctx);

        // Limit positioning to vertical screen size
        clamp(
            &mut self.position.y,
            PLAYER_H_HALF,
            screen_h - PLAYER_H_HALF,
        );
        // Limit positioning to horizontal screen size
        clamp(
            &mut self.position.x,
            PLAYER_W_HALF,
            screen_w - PLAYER_W_HALF,
        );
    }
}
