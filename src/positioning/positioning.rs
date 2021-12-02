use ggez::{event::KeyCode, graphics, Context};

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn from_f32(tuple: (f32, f32)) -> Position {
        Position {
            x: tuple.0,
            y: tuple.1,
        }
    }

    pub fn clamp_self(&mut self, object_size: &Size, area_coordinates: &Position) {
        clamp(
            &mut self.x,
            object_size.w_half(),
            area_coordinates.x - object_size.w_half(),
        );
        clamp(
            &mut self.y,
            object_size.h_half(),
            area_coordinates.y - object_size.h_half(),
        );
    }
}

// We use clamping to limit position to a given area. Clamping merely moves the point to the nearest available value
pub fn clamp_object(
    object_position: &mut Position,
    object_size: &Size,
    area_coordinates: &Position,
) {
    clamp(
        &mut object_position.x,
        object_size.w_half(),
        area_coordinates.x - object_size.w_half(),
    );
    clamp(
        &mut object_position.y,
        object_size.h_half(),
        area_coordinates.y - object_size.h_half(),
    );
}

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

#[derive(Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn h_half(&self) -> f32 {
        self.height * 0.5
    }
    pub fn w_half(&self) -> f32 {
        self.width * 0.5
    }
}

pub trait Sizable {
    fn get_size(&self) -> &Size;
    fn get_position(&self) -> &Position;
}

#[derive(Copy, Clone)]

pub struct Body {
    pub position: Position,
    pub size: Size,
    pub speed: f32,
}

impl Sizable for Body {
    fn get_size(&self) -> &Size {
        &self.size
    }

    fn get_position(&self) -> &Position {
        &self.position
    }
}

impl Body {
    pub fn update_position(&mut self, direction: KeyCode, ctx: &mut Context) {

        let dt = ggez::timer::delta(ctx).as_secs_f32();
        match direction {
            KeyCode::Up => self.position.y -= self.speed * dt,
            KeyCode::Down => self.position.y += self.speed * dt,
            KeyCode::Left => self.position.x -= self.speed * dt,
            KeyCode::Right => self.position.x += self.speed * dt,
            _ => (),
        }

        self.position.clamp_self(
            &self.size,
            &Position::from_f32(graphics::drawable_size(ctx)),
        );

    }
}
