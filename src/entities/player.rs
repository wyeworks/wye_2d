use ggez::event::*;
use ggez::graphics::*;
use ggez::*;

use crate::positioning::positioning::*;

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_W: f32 = 25.0;
const PLAYER_H: f32 = 25.0;
pub struct Player {
    pub position: Position,
    pub size: Size,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Position { x: 200.0, y: 300.0 },
            size: Size {
                width: PLAYER_W,
                height: PLAYER_H,
            },
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(
            self.position.x - self.size.w_half(),
            self.position.y - self.size.h_half(),
            self.size.width,
            self.size.height,
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

        self.position.clamp_self(
            &self.size,
            &Position::from_f32(graphics::drawable_size(ctx)),
        );
    }
}
