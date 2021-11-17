use ggez::graphics::*;
use ggez::*;

use crate::Position;

const DESK_H: f32 = 70.0;
const DESK_W: f32 = 150.0;
const DESK_H_HALF: f32 = DESK_H * 0.5;
const DESK_W_HALF: f32 = DESK_W * 0.5;
pub struct Desk {
    pub position: Position,
}

impl Desk {
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(
            self.position.x - DESK_W_HALF,
            self.position.y - DESK_H_HALF,
            DESK_W,
            DESK_H,
        );

        let rect_mesh =
            graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, graphics::Color::WHITE)?;

        graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

        Ok(())
    }
}
