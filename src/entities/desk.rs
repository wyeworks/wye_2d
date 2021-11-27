use ggez::graphics::*;
use ggez::*;

use crate::positioning::positioning::*;

const DESK_H: f32 = 70.0;
const DESK_W: f32 = 150.0;
pub struct Desk {
    pub position: Position,
    pub size: Size,
}

impl Desk {
    pub fn new(position: Position) -> Self {
        Desk {
            position,
            size: Size {
                width: DESK_W,
                height: DESK_H,
            },
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(
            self.position.x - self.size.w_half(),
            self.position.y - self.size.h_half(),
            DESK_W,
            DESK_H,
        );

        let rect_mesh =
            graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, graphics::Color::WHITE)?;

        graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

        Ok(())
    }
}
