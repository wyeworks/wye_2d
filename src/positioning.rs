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
    object_size: (f32, f32),
    area_coordinates: Position,
) {
    let object_w_half = object_size.0 * 0.5;
    let object_h_half = object_size.1 * 0.5;
    clamp(
        &mut object_position.x,
        object_size.0 * 0.5,
        area_coordinates.x - object_w_half,
    );
    clamp(
        &mut object_position.y,
        object_h_half,
        area_coordinates.y - object_h_half,
    );
}

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}
