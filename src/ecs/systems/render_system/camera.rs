use ggez::{Context};
use super::super::physics_system::physics::{Physics, Position, Size, Direction};
use super::super::super::utils::constants::{DEFAULT_CAMERA_W, DEFAULT_CAMERA_H, DEFAULT_CAMERA_SPEED, DEFAULT_CAMERA_OFFSET};

#[derive(Copy, Clone)]
pub struct Camera {
    pub position: Position,
    pub size: Size,
    pub speed: f32,
}

impl Camera {
    pub fn new(position: Position) -> Self {
        Camera {
            position,
            size: Size {
                width: DEFAULT_CAMERA_W,
                height: DEFAULT_CAMERA_H,
            },
            speed: DEFAULT_CAMERA_SPEED,
        }
    }

    pub fn world_to_screen(&self, world_position: &Position) -> Position {
        Position {
            x: world_position.x - (self.position.x - self.size.w_half()),
            y: world_position.y - (self.position.y - self.size.h_half()),
        }
    }

    pub fn update_position(&mut self, direction: Direction, ctx: &mut Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        match direction {
            Direction::Up => self.position.y -= self.speed * dt,
            Direction::Down => self.position.y += self.speed * dt,
            Direction::Left => self.position.x -= self.speed * dt,
            Direction::Right => self.position.x += self.speed * dt
        }
    }

    pub fn is_player_approaching_camera_edge(
        &self,
        player_physics: &Physics
    ) -> bool {
        let player_camera_position = self.world_to_screen(&player_physics.position);
        let camera_w_third = self.size.width / 3.0;
        let camera_h_third = self.size.height / 3.0;

        let left_boundry = camera_w_third;
        let top_boundry = camera_h_third;
        let bottom_boundry = self.size.height - camera_h_third;
        let right_boundry = self.size.width - camera_h_third;

        return (player_camera_position.x < left_boundry && player_physics.direction == Some(Direction::Left))
            || (player_camera_position.y < top_boundry && player_physics.direction == Some(Direction::Up))
            || (player_camera_position.x > right_boundry && player_physics.direction == Some(Direction::Right))
            || (player_camera_position.y > bottom_boundry && player_physics.direction == Some(Direction::Down));
    }

    pub fn is_within_world_bounds(&self, world_size: &Size, direction: Direction) -> bool {
        let camera_w_half = self.size.w_half();
        let camera_h_half = self.size.h_half();

        let left_boundry = self.position.x - camera_w_half <= -DEFAULT_CAMERA_OFFSET;
        let top_boundry = self.position.y - camera_h_half <= -DEFAULT_CAMERA_OFFSET;

        let right_boundry =
            self.position.x + camera_w_half >= world_size.width + DEFAULT_CAMERA_OFFSET;
        let bottom_boundry =
            self.position.y + camera_h_half >= world_size.height + DEFAULT_CAMERA_OFFSET;

        !((left_boundry && direction == Direction::Left)
            || (top_boundry && direction == Direction::Up)
            || (bottom_boundry && direction == Direction::Down)
            || (right_boundry && direction == Direction::Right))
    }

    pub fn maybe_update(&mut self, ctx: &mut Context, player_physics: &Physics, world_size: &Size) {
        let should_update_camera = self.is_player_approaching_camera_edge(player_physics)
            && self.is_within_world_bounds(world_size, player_physics.direction.unwrap());
    
        if should_update_camera {
            self.update_position(player_physics.direction.unwrap(), ctx);
        }
    }
}
