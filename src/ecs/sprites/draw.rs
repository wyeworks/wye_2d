use super::super::systems::{physics_system::physics::*, render_system::camera::Camera};
use ggez::graphics::spritebatch::SpriteBatch;

pub trait Draw {
    fn draw(&mut self, batch: &mut SpriteBatch, camera: &Camera, physics: &Physics, frames: usize);
}

pub trait DrawComponent {
    type Component;

    fn draw_component(&mut self, batch: &mut SpriteBatch, camera: &Camera, physics: &Physics, frames: usize, component: &Self::Component);
}