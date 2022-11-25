use super::super::systems::physics_system::physics::Position;

pub const HUMANOID_W: f32 = 30.0;
pub const HUMANOID_H: f32 = 60.0;
pub const DESK_H: f32 = 70.0;
pub const DESK_W: f32 = 150.0;

pub const INITIAL_PLAYER_POS: Position = Position {
    x: 1000.0,
    y: 500.0,
};
pub const INITIAL_PLAYER_SPEED: f32 = 125.0;

pub const INTIAL_WORLD_W: f32 = 2000.0;
pub const INTIAL_WORLD_H: f32 = 1000.0;

pub const DEFAULT_WINDOW_W: f32 = 1600.0;
pub const DEFAULT_WINDOW_H: f32 = 800.0;

pub const DEFAULT_CAMERA_OFFSET: f32 = 80.0;
pub const DEFAULT_CAMERA_SPEED: f32 = 125.0;
pub const DEFAULT_CAMERA_W: f32 = DEFAULT_WINDOW_W;
pub const DEFAULT_CAMERA_H: f32 = DEFAULT_WINDOW_H;

pub const NPC_COUNT: i32 = 4;

// initial live count