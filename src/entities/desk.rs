use ggez::graphics::*;
use ggez::*;

use crate::Position;

const DESK_HEIGHT: f32 = 70.0;
const DESK_WIDTH: f32 = 150.0;
pub struct Desk {
    pub position: Position,
}

impl Desk {
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(self.position.x, self.position.y, DESK_WIDTH, DESK_HEIGHT);

        let rect_mesh =
            graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, graphics::Color::WHITE)?;

        graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

        graphics::present(ctx)?;

        Ok(())
    }
}
