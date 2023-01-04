use crate::ecs::{
    atlas::{self, Atlas},
    components::npc::Npc,
    sprites::npc_sprite::NpcSprite,
    sprites::player_sprite::PlayerSprite,
    sprites::tile_sprite::{create_tiles, TileSprite},
    systems::{
        input_system::{self, interaction::*},
        physics_system::{self, physics::*},
        render_system::{self, camera::*},
    },
    utils::{constants::*, npcs_json_loader::load_npcs},
};
use ggez::{event::*, graphics::spritebatch::SpriteBatch, mint::Vector2, *};

pub type EntityIndex = usize;

pub struct GameState {
    pub physics_components: Vec<Option<Physics>>,
    pub npcs_components: Vec<Option<Npc>>,
    pub player_physics: Physics,
    pub current_interaction: Option<Interaction>,
    pub npcs_interactions: Vec<Option<Interaction>>,
    pub camera: Camera,
    pub world_size: Size,
    tiles: Vec<Box<TileSprite>>,
    player_sprite: PlayerSprite,
    player_sprite_batch: SpriteBatch,
    npcs_sprite: NpcSprite,
    npcs_sprite_batch: SpriteBatch,
    world_sprite_batch: SpriteBatch,
    frames: usize,
}

impl ggez::event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let player_mov_actions = input_system::player_movements(ctx);

        match self.current_interaction {
            Some(_) => (),
            None => {
                self.player_physics = physics_system::update_player_physics(
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

        render_system::draw_tiles(
            ctx,
            &self.camera,
            &mut self.tiles,
            &mut self.world_sprite_batch,
            draw_param,
        )?;
        render_system::draw_world_bounds(ctx, &self.camera, &self.world_size)?;
        render_system::draw_player(
            ctx,
            &self.camera,
            &self.player_physics,
            &mut self.player_sprite_batch,
            &mut self.player_sprite,
            self.frames,
            draw_param,
        )?;
        render_system::draw_objects(ctx, &self.camera, &self.physics_components)?;
        render_system::draw_npcs(
            ctx,
            &self.camera,
            &self.physics_components,
            &self.npcs_components,
            &mut self.npcs_sprite_batch,
            &mut self.npcs_sprite,
            draw_param,
        )?;
        render_system::draw_interactions(
            ctx,
            &self.camera.size,
            &self.npcs_components,
            &self.current_interaction,
            &self.player_physics.current_focus,
        )?;

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
        let physics_components = Vec::new();
        let player_physics = physics_system::initial_player_physics();
        let npcs_interactions = Vec::new();

        let player_atlas =
            Atlas::parse_atlas_json(std::path::Path::new("src/resources/player64.json"));
        let player_sprite_batch = atlas::create_batch_sprite(ctx, "/player64.png".to_string());

        let npcs_atlas = Atlas::parse_atlas_json(std::path::Path::new("src/resources/npcs64.json"));
        let npcs_sprite_batch = atlas::create_batch_sprite(ctx, "/npcs64.png".to_string());

        let world_atlas =
            Atlas::parse_atlas_json(std::path::Path::new("src/resources/world_atlas.json"));
        let world_sprite_batch = atlas::create_batch_sprite(ctx, "/world_atlas.png".to_string());

        let camera = Camera::new(player_physics.position.clone());

        let mut game_state = GameState {
            physics_components,
            npcs_components,
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
            world_sprite_batch,
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
                Some(physics_system::generate_npc_physics()),
                Some(Npc {
                    id: npc_data.id,
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

// TO DO
// - Sprites
//  - Interactions boxes
//   - dialog box, avatar box
//  - desks
//  - npcs
// - re review logic
