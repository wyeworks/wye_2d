// Position of the elements in the screen, this will be used by all entities
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
