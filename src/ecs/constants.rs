//use ggez::graphics;

use super::systems::physics_system::positioning::positioning::Position;

pub const HUMANOID_W: f32 = 25.0;
pub const HUMANOID_H: f32 = 25.0;
pub const DESK_H: f32 = 70.0;
pub const DESK_W: f32 = 150.0;

pub const INITIAL_PLAYER_POS: Position = Position { x: 500.0, y: 600.0 };
pub const INITIAL_PLAYER_SPEED: f32 = 125.0;
//pub const INITIAL_PLAYER_COLOR: graphics::Color = graphics::Color::from_rgb(0, 171, 169);

//pub const INITIAL_WW_COLOR: graphics::Color = graphics::Color::from_rgb(112, 111, 211);
