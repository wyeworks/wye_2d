use ggez::{event::KeyCode, graphics, Context};
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

// Position and Size could be replaced by a Rect
#[derive(Copy, Clone, Debug)]
pub struct Physics {
    pub position: Position,
    pub size: Size,
    pub speed: f32,
    pub color: graphics::Color,
    pub direction: Option<Direction>,
    pub walking: bool,
    pub current_focus: Option<usize>,
}

impl Physics {
    pub fn new(
        position: Position,
        size: Size,
        speed: f32,
        color: graphics::Color,
        direction: Option<Direction>,
        current_focus: Option<usize>,
    ) -> Self {
        Physics {
            position,
            size,
            speed,
            color,
            direction,
            walking: false,
            current_focus
        }
    }

    pub fn update_position(&mut self, ctx: &mut Context, direction: KeyCode, world_size: &Size) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        match direction {
            KeyCode::Up => self.position.y -= self.speed * dt,
            KeyCode::Down => self.position.y += self.speed * dt,
            KeyCode::Left => self.position.x -= self.speed * dt,
            KeyCode::Right => self.position.x += self.speed * dt,
            _ => (),
        }
        self.walking = true;
        self.position.clamp_self(
            &self.size,
            &Position {
                x: world_size.width,
                y: world_size.height,
            },
        );
    }
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug, Hash, Eq, Display, EnumString, IntoStaticStr, EnumIter, PartialEq)]
pub enum Direction {
    #[strum(serialize = "up")]
    Up,
    #[strum(serialize = "down")]
    Down,
    #[strum(serialize = "left")]
    Left,
    #[strum(serialize = "right")]
    Right,
}

impl Direction {
    pub fn to_index(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}
