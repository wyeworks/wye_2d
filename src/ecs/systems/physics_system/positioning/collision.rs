use super::positioning::*;
use crate::ecs::game_state::EntityIndex;
use std::time::Instant;

pub fn objects_collide(a: &Physics, b: &Physics) -> bool {
    let collision = a.get_position().x - a.get_size().w_half()
        < b.get_position().x + b.get_size().w_half()
        && a.get_position().x + a.get_size().w_half() > b.get_position().x - b.get_size().w_half()
        && a.get_position().y - a.get_size().h_half() < b.get_position().y + b.get_size().h_half()
        && a.get_size().h_half() + a.get_position().y > b.get_position().y - b.get_size().h_half();
    return collision;
}

#[derive(Copy, Clone)]
pub struct Interaction {
    pub making_contact: bool,
    pub began_at: Instant,
    pub interacting_with: EntityIndex,
}

impl Interaction {
    pub fn new(interacting_with: EntityIndex) -> Interaction {
        Interaction {
            making_contact: true,
            began_at: Instant::now(),
            interacting_with,
        }
    }
}
