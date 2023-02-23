use super::{atlas::Atlas, utils::constants::*};
use super::{
    atlas::{self},
    sprites::office_sprite::OfficeSprite,
    sprites::player_sprite::PlayerSprite,
    sprites::npc_sprite::NpcSprite,
    sprites::tile_sprite::{create_tiles, TileSprite},
    systems::{
        input_system::input_system,
        input_system::interaction::*,
        physics_system::physics::*,
        physics_system::physics_system::*,
        render_system::{camera::*, render_system::*},
    },
};
use super::{components::npc::Npc, components::desk::Desk, utils::npcs_json_loader::load_npcs};
use ggez::*;
use ggez::{event::*, graphics::spritebatch::SpriteBatch, mint::Vector2};
use rand::Rng;

pub type EntityIndex = usize;

// #[derive(Copy, Clone)]
pub struct GameState {
    pub physics_components: Vec<Option<Physics>>,
    pub npcs_components: Vec<Option<Npc>>,
    pub player_physics: Physics,
    pub current_interaction: Option<Interaction>,
    pub npcs_interactions: Vec<Option<Interaction>>,
    pub desk_components: Vec<Option<Desk>>,
    pub camera: Camera,
    pub world_size: Size,
    tiles: Vec<Box<TileSprite>>,
    player_sprite: PlayerSprite,
    player_sprite_batch: SpriteBatch,
    npcs_sprite: NpcSprite,
    npcs_sprite_batch: SpriteBatch,
    floor_sprite_batch: SpriteBatch,
    office_sprite_batch: SpriteBatch,
    office_sprite: OfficeSprite,
    frames: usize,
}

impl ggez::event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let player_mov_actions = input_system::player_movements(ctx);

        match self.current_interaction {
            Some(_) => (),
            None => {
                self.player_physics = update_player_physics(
                    ctx,
                    &player_mov_actions,
                    &self.player_physics,
                    &self.physics_components,
                    &self.world_size,
                );
                self.camera
                    .maybe_update(ctx, &self.player_physics, &self.world_size);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(130, 90, 44));

        let draw_param = graphics::DrawParam::new().scale(Vector2 { x: 1.0, y: 1.0 });

        draw_tiles(
            ctx,
            &self.camera,
            &mut self.tiles,
            &mut self.floor_sprite_batch,
            draw_param,
        )?;
        draw_world_bounds(ctx, &self.camera, &self.world_size)?;
        draw_npcs(ctx, &self.camera, &self.physics_components, &self.npcs_components, &mut self.npcs_sprite_batch, &mut self.npcs_sprite, draw_param)?;
        draw_interactions(ctx, &self.camera.size, &self.npcs_components, &self.current_interaction, &self.player_physics.current_focus)?;
        draw_sprite(
            ctx,
            &self.camera,
            &self.player_physics,
            &mut self.player_sprite_batch,
            &mut self.player_sprite,
            self.frames,
            draw_param,
        )?;


        for (index, component) in self.physics_components.iter().enumerate() {
            match component {
                Some(physics) => match index {
                    i if !self.desk_components[i].is_none() => {
                        let desk: &Desk = &self.desk_components[i].unwrap();
                        draw_sprite_component(
                            ctx,
                            &self.camera,
                            physics,
                            &mut self.office_sprite_batch,
                            &mut self.office_sprite,
                            self.frames,
                            draw_param,
                            desk
                        )?;
                    }
                    _ => (),
                },
                None => (),
            }
        }

        graphics::present(ctx)?;

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
        let desk_components = Vec::new();
        let physics_components = Vec::new();
        let player_physics = initial_player_physics();
        let npcs_interactions = Vec::new();

        let player_atlas =
            Atlas::parse_atlas_json(std::path::Path::new("src/resources/player64.json"));
        let player_sprite_batch = atlas::create_batch_sprite(ctx, "/player64.png".to_string());

        let npcs_atlas =
            Atlas::parse_atlas_json(std::path::Path::new("src/resources/npcs64.json"));
        let npcs_sprite_batch = atlas::create_batch_sprite(ctx, "/npcs64.png".to_string());

        let floor_atlas = Atlas::parse_atlas_json(std::path::Path::new("src/resources/floor.json"));
        let floor_sprite_batch = atlas::create_batch_sprite(ctx, "/floor.png".to_string());

        let office_atlas =
            Atlas::parse_atlas_json(std::path::Path::new("src/resources/office.json"));
        let office_sprite_batch = atlas::create_batch_sprite(ctx, "/office.png".to_string());

        let camera = Camera::new(player_physics.position.clone());

        let mut game_state = GameState {
            physics_components,
            npcs_components,
            desk_components,
            player_physics,
            current_interaction: None,
            npcs_interactions,
            camera,
            world_size: Size {
                width: INTIAL_WORLD_W,
                height: INTIAL_WORLD_H,
            },
            player_sprite: PlayerSprite::new(&player_atlas),
            player_sprite_batch,
            npcs_sprite: NpcSprite::new(&npcs_atlas),
            npcs_sprite_batch,
            tiles: create_tiles(&floor_atlas),
            floor_sprite_batch,
            office_sprite_batch,
            office_sprite: OfficeSprite::new(&office_atlas),
            frames: 0,
        };
        game_state.load_initial_components();
        game_state
    }

    // Components loading
    fn load_initial_components(&mut self) {
        self.add_npcs();
        self.add_desks();
    }

    fn add_npcs(&mut self) {
        let npcs = load_npcs();
        for npc_data in npcs.iter() {
            self.add_entity(
                Some(generate_npc_physics()),
                Some(Npc {
                    id: npc_data.id,
                    name: npc_data.name.clone(),
                }),
                npc_data.main_interaction.clone(),
                None,
            );
        }
    }

    fn add_desks(&mut self) {
        for animation_id in 2..6 {
            let desk_type = rand::thread_rng().gen_range(0..=1);
            let object_physics = Some(Physics::new(
                Position {
                    x: 200.0,
                    y: animation_id as f32 * 130.0,
                },
                Size {
                    height: DESK_H,
                    width: DESK_W,
                },
                0.0,
                graphics::Color::WHITE,
                Some(Direction::Up),
                None,
            ));
            self.add_entity(
                object_physics,
                None,
                None,
                Some(Desk {
                    desk_type,
                    animation_id
                }),
            );
        }
    }

    fn add_entity(
        &mut self,
        physics: Option<Physics>,
        npc: Option<Npc>,
        interaction: Option<Interaction>,
        desk: Option<Desk>,
    ) {
        self.physics_components.push(physics);
        self.npcs_components.push(npc);
        self.npcs_interactions.push(interaction);
        self.desk_components.push(desk);
    }
}
