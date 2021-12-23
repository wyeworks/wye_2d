use super::components::{npc::Npc, player::Player};
use super::systems::physics_system::physics_system::*;
use super::{constants::*, systems::*};
use crate::ecs::systems::physics_system::positioning::{collision::Interaction, positioning::*};
use ggez::*;

pub type EntityIndex = usize;

pub struct World {
    player_components: Vec<Option<Player>>,
    physics_components: Vec<Option<Physics>>,
    npcs_components: Vec<Option<Npc>>,
    players: Vec<EntityIndex>,
}

impl World {
    pub fn new() -> World {
        let player_components = Vec::new();
        let npcs_components = Vec::new();
        let physics_components = Vec::new();
        let players = Vec::new();

        World {
            players,
            physics_components,
            npcs_components,
            player_components,
        }
    }

    pub fn load_initial_components(&mut self) {
        self.add_player();
        self.add_npcs();
        self.add_innanimate_objects();
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        update_physics(
            &mut self.physics_components,
            &mut self.npcs_components,
            &mut self.player_components,
            ctx,
        )?;
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        render_system::render(
            &self.physics_components,
            &self.player_components,
            &self.npcs_components,
            ctx,
        )?;
        Ok(())
    }

    pub fn begin_interaction(&mut self, _ctx: &mut Context) {
        for player in self.player_components.iter_mut() {
            match player {
                Some(player) => {
                    let current_focus = player.current_focus;
                    match current_focus {
                        Some(focused_entity_id) => {
                            player.interacting = Some(Interaction::new(focused_entity_id));
                        }
                        None => (),
                    }
                }
                None => (),
            }
        }
    }

    pub fn add_player(&mut self) {
        let player_component: Option<Player> = Some(Player {
            interacting: None,
            current_focus: None,
        });

        let player_physics = Some(generate_physics(Entity::Player));
        self.add_entity(player_physics, player_component, None);
    }

    pub fn add_npcs(&mut self) {
        let npcs = get_wyeworkers_npcs();
        for npc_data in npcs.iter() {
            self.add_entity(
                Some(generate_physics(Entity::Npc)),
                None,
                Some(Npc {
                    name: npc_data.to_owned(),
                }),
            );
        }
    }

    pub fn add_innanimate_objects(&mut self) {
        self.add_first_desk_row();
    }

    pub fn add_first_desk_row(&mut self) {
        for n in 2..6 {
            let object_physics = Some(Physics::new(
                Position {
                    x: 200.0,
                    y: n as f32 * 120.0,
                },
                Size {
                    height: DESK_H,
                    width: DESK_W,
                },
                0.0,
                graphics::Color::WHITE,
            ));
            self.add_entity(object_physics, None, None)
        }
    }

    pub fn add_entity(
        &mut self,
        physics: Option<Physics>,
        player: Option<Player>,
        npc: Option<Npc>,
    ) {
        self.physics_components.push(physics);
        self.npcs_components.push(npc);
        self.player_components.push(player);
        match player {
            Some(_) => self.players.push(self.player_components.len() - 1),
            None => (),
        }
    }
}

pub fn get_wyeworkers_npcs() -> Vec<String> {
    let mut wyeworkers = Vec::new();
    wyeworkers.push("Juan".to_string());
    wyeworkers.push("Andrés".to_string());
    wyeworkers.push("Nico".to_string());
    wyeworkers.push("Mauri".to_string());
    wyeworkers
}
