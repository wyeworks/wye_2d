use crate::ecs::systems::physics_system::positioning::collision::Interaction;
use crate::ecs::world::EntityIndex;

#[derive(Copy, Clone)]
pub struct Player {
    pub interacting: Option<Interaction>,
    pub current_focus: Option<EntityIndex>,
}
