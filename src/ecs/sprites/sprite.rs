use ggez::{graphics, mint::{Point2, Vector2}};


#[derive(Clone, Debug)]
pub struct Sprite {
    /// The square that we want to cut out of the texture atlas.
    pub rect: graphics::Rect,
    pub scale: Vector2<f32>,
    pub width: f32,
    pub height: f32,
}

impl Sprite {
    pub fn new(rect: graphics::Rect, width: f32, height: f32) -> Self {
        Self {
            rect,
            scale: Vector2 { x: 1.0, y: 1.0 },
            width,
            height,
        }
    }

    pub fn draw_params(&self, pos: Point2<f32>, scale: Vector2<f32>) -> graphics::DrawParam {
        graphics::DrawParam::new()
            .src(self.rect.clone())
            .scale(scale)
            .dest(pos)
    }

    pub fn get_bound_box(&self) -> graphics::Rect {
        let mut r = graphics::Rect::new(0.0, 0.0, self.width, self.height);
        r.scale(self.scale.x, self.scale.y);
        r
    }
}
