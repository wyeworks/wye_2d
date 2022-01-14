use ggez::{event::KeyCode, Context};

use crate::ecs::constants::DEFAULT_CAMERA_OFFSET;

use super::physics_system::positioning::positioning::{Position, Size};

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
                width: 1600.0,
                height: 800.0,
            },
            speed: 125.0,
        }
    }

    pub fn world_to_screen(&self, world_position: &Position) -> Position {
        Position {
            x: world_position.x - (self.position.x - self.size.w_half()),
            y: world_position.y - (self.position.y - self.size.h_half()),
        }
    }

    pub fn update_position(&mut self, direction: KeyCode, ctx: &mut Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        match direction {
            KeyCode::Up => self.position.y -= self.speed * dt,
            KeyCode::Down => self.position.y += self.speed * dt,
            KeyCode::Left => self.position.x -= self.speed * dt,
            KeyCode::Right => self.position.x += self.speed * dt,
            _ => (),
        }
    }

    pub fn is_player_approaching_camera_edge(
        &self,
        player_position: &Position,
        direction: KeyCode,
    ) -> bool {
        let player_camera_position = self.world_to_screen(player_position);
        let camera_w_third = self.size.width / 3.0;
        let camera_h_third = self.size.height / 3.0;

        let left_boundry = camera_w_third;
        let top_boundry = camera_h_third;
        let bottom_boundry = self.size.height - camera_h_third;
        let right_boundry = self.size.width - camera_h_third;

        return (player_camera_position.x < left_boundry && direction == KeyCode::Left)
            || (player_camera_position.y < top_boundry && direction == KeyCode::Up)
            || (player_camera_position.x > right_boundry && direction == KeyCode::Right)
            || (player_camera_position.y > bottom_boundry && direction == KeyCode::Down);
    }

    pub fn is_within_world_bounds(&self, world_size: &Size, direction: KeyCode) -> bool {
        let camera_w_half = self.size.w_half();
        let camera_h_half = self.size.h_half();

        let left_boundry = self.position.x - camera_w_half <= -DEFAULT_CAMERA_OFFSET;
        let top_boundry = self.position.y - camera_h_half <= -DEFAULT_CAMERA_OFFSET;

        let right_boundry =
            self.position.x + camera_w_half >= world_size.width + DEFAULT_CAMERA_OFFSET;
        let bottom_boundry =
            self.position.y + camera_h_half >= world_size.height + DEFAULT_CAMERA_OFFSET;

        !((left_boundry && direction == KeyCode::Left)
            || (top_boundry && direction == KeyCode::Up)
            || (bottom_boundry && direction == KeyCode::Down)
            || (right_boundry && direction == KeyCode::Right))
    }
}
