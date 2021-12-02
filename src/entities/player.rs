use ggez::graphics::*;
use ggez::*;

use crate::positioning::positioning::*;

const PLAYER_W: f32 = 25.0;
const PLAYER_H: f32 = 25.0;
pub struct Player {
    pub body: Body,
}

impl Player {
    pub fn new() -> Self {
        Player {
            body: Body {
                position: Position { x: 600.0, y: 500.0 },
                size: Size {
                    width: PLAYER_W,
                    height: PLAYER_H,
                },
                speed: 150.0
            },
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(
            self.body.position.x - self.body.size.w_half(),
            self.body.position.y - self.body.size.h_half(),
            self.body.size.width,
            self.body.size.height,
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
}
