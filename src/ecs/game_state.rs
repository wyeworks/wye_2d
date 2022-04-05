use super::{atlas::Atlas, utils::constants::*};
use super::{
    atlas::{self},
    sprites::player_sprite::PlayerSprite,
    sprites::tile_sprite::{create_tiles, TileSprite},
    systems::{
        input_system::input_system,
        physics_system::physics_system::*,
        render_system::{camera_system::Camera, render_system},
    },
};
use super::{components::npc::Npc, utils::npcs_json_loader::load_npcs};
use crate::ecs::systems::physics_system::positioning::{collision::Interaction, positioning::*};
use ggez::*;
use ggez::{event::*, graphics::spritebatch::SpriteBatch};

pub type EntityIndex = usize;

pub struct GameState {
    physics_components: Vec<Option<Physics>>,
    pub npcs_components: Vec<Option<Npc>>,
    player_physics: Physics,
    pub current_interaction: Option<Interaction>,
    pub current_focus: Option<EntityIndex>,
    pub npcs_interactions: Vec<Option<Interaction>>,
    camera: Camera,
    world_size: Size,
    tiles: Vec<Box<TileSprite>>,
    player_sprite_batch: SpriteBatch,
    world_sprite_batch: SpriteBatch,
    player_sprite: PlayerSprite,
    frames: usize,
}

impl ggez::event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let player_mov_actions = input_system::player_movements(ctx);
        match self.current_interaction {
            Some(_) => (),
            None => {
                update_player_physics(
                    ctx,
                    &self.physics_components,
                    &mut self.current_focus,
                    &mut self.player_physics,
                    &player_mov_actions,
                    &mut self.camera,
                    &self.world_size,
                )?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(130, 90, 44));

        render_system::render(
            ctx,
            &self.physics_components,
            &self.player_physics,
            &self.npcs_components,
            &self.current_interaction,
            &self.current_focus,
            &mut self.camera,
            &mut self.player_sprite,
            &mut self.player_sprite_batch,
            &mut self.tiles,
            &mut self.world_sprite_batch,
            &self.world_size,
            self.frames,
        )?;
        graphics::present(ctx)?;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        self.frames += 1;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        self.current_interaction = input_system::key_down_event_interaction(self, key);
    }
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameState {
        let npcs_components = Vec::new();
        let physics_components = Vec::new();
        let player_physics = generate_physics(Entity::Player);
        let npcs_interactions = Vec::new();

        let player_atlas =
            Atlas::parse_atlas_json(std::path::Path::new("src/resources/player64.json"));
        let player_sprite_batch = atlas::create_batch_sprite(ctx, "/player64.png".to_string());

        let world_atlas =
            Atlas::parse_atlas_json(std::path::Path::new("src/resources/world_atlas.json"));
        let world_sprite_batch = atlas::create_batch_sprite(ctx, "/world_atlas.png".to_string());

        let camera = Camera::new(player_physics.position.clone());

        let mut game_state = GameState {
            physics_components,
            npcs_components,
            player_physics,
            current_interaction: None,
            current_focus: None,
            npcs_interactions,
            camera,
            world_size: Size {
                width: INTIAL_WORLD_W,
                height: INTIAL_WORLD_H,
            },
            player_sprite_batch,
            world_sprite_batch,
            player_sprite: PlayerSprite::new(&player_atlas),
            tiles: create_tiles(&world_atlas),
            frames: 0,
        };
        game_state.load_initial_components();
        game_state
    }

    // Components loading
    fn load_initial_components(&mut self) {
        self.add_npcs();
        self.add_first_desk_row();
    }

    fn add_npcs(&mut self) {
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

    fn add_first_desk_row(&mut self) {
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
                None,
            ));
            self.add_entity(object_physics, None, None);
        }
    }

    fn add_entity(
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
