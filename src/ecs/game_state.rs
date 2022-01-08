use super::systems::physics_system::physics_system::*;
use super::{components::npc::Npc, npcs_loader::load_npcs};
use super::{constants::*, systems::*};
use crate::ecs::systems::physics_system::positioning::{collision::Interaction, positioning::*};
use ggez::event::*;
use ggez::*;

pub type EntityIndex = usize;

pub struct GameState {
    physics_components: Vec<Option<Physics>>,
    npcs_components: Vec<Option<Npc>>,
    player_physics: Physics,
    current_interaction: Option<Interaction>,
    current_focus: Option<EntityIndex>,
    npcs_interactions: Vec<Option<Interaction>>,
}

impl ggez::event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(130, 90, 44));
        self.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match self.current_interaction {
            Some(_) => self.interaction_input_handler(key),
            None => match key {
                KeyCode::Space => {
                    *self = GameState::new();
                    self.load_initial_components();
                }
                KeyCode::Return => {
                    self.begin_interaction();
                }

                _ => (),
            },
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        let npcs_components = Vec::new();
        let physics_components = Vec::new();
        let player_physics = generate_physics(Entity::Player);
        let npcs_interactions = Vec::new();

        GameState {
            physics_components,
            npcs_components,
            player_physics,
            current_interaction: None,
            current_focus: None,
            npcs_interactions,
        }
    }

    pub fn load_initial_components(&mut self) {
        self.add_npcs();
        self.add_innanimate_objects();
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let player_mov_actions = player_input_system::handle_input(ctx);
        match self.current_interaction {
            Some(_) => (),
            None => {
                update_player_physics(
                    &self.physics_components,
                    &mut self.current_focus,
                    &mut self.player_physics,
                    &player_mov_actions,
                    ctx,
                )?;
            }
        }
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        render_system::render(
            &self.physics_components,
            &self.player_physics,
            &self.npcs_components,
            &self.current_interaction,
            &self.current_focus,
            ctx,
        )?;
        Ok(())
    }

    pub fn begin_interaction(&mut self) {
        match self.current_focus {
            Some(focused_entity_id) => match &self.npcs_components[focused_entity_id] {
                Some(_) => {
                    self.current_interaction = self.npcs_interactions[focused_entity_id].clone()
                }
                None => (),
            },
            None => (),
        }
    }

    fn interaction_input_handler(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Down | KeyCode::Return => self.update_interaction(key),
            KeyCode::Escape => self.end_interaction(),
            _ => (),
        }
    }

    fn update_interaction(&mut self, action: KeyCode) {
        let mut interaction = self.current_interaction.as_mut().unwrap();
        match &interaction.sub_interactions {
            Some(sub_interactions) => match action {
                KeyCode::Up => {
                    if interaction.hovered_option != 0 {
                        interaction.hovered_option -= 1;
                    }
                }
                KeyCode::Down => {
                    if interaction.hovered_option < interaction.options.as_ref().unwrap().len() - 1
                    {
                        interaction.hovered_option += 1;
                    }
                }
                KeyCode::Return => {
                    self.current_interaction =
                        Some(sub_interactions[interaction.hovered_option].clone());
                }
                _ => (),
            },
            None => (),
        }
    }

    pub fn end_interaction(&mut self) {
        self.current_interaction = None;
        self.current_focus = None;
    }

    pub fn add_npcs(&mut self) {
        let npcs = load_npcs();
        for npc_data in npcs.iter() {
            self.add_entity(
                Some(generate_physics(Entity::Npc)),
                Some(Npc {
                    name: npc_data.name.clone(),
                }),
                npc_data.main_interaction.clone(),
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
            self.add_entity(object_physics, None, None);
        }
    }

    pub fn add_entity(
        &mut self,
        physics: Option<Physics>,
        npc: Option<Npc>,
        interaction: Option<Interaction>,
    ) {
        self.physics_components.push(physics);
        self.npcs_components.push(npc);
        self.npcs_interactions.push(interaction);
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
