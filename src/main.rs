#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use ecs::{
    constants::{DEFAULT_WINDOW_H, DEFAULT_WINDOW_W},
    game_state::GameState,
};
use ggez::*;
use ggez::{conf::FullscreenType, graphics::spritebatch::SpriteBatch};

pub mod ecs {
    pub mod atlas;
    pub mod constants;
    pub mod game_state;
    pub mod npcs_loader;
    pub mod player_sprite;
    pub mod tile;
    pub mod components {
        pub mod npc;
    }
    pub mod systems {
        pub mod physics_system {
            pub mod physics_system;
            pub mod positioning {
                pub mod collision;
                pub mod positioning;
            }
        }
        pub mod camera_system;
        pub mod player_input_system;
        pub mod render_system;
    }
}

fn main() -> GameResult {
    let c = conf::Conf::new();

    let window_mode = ggez::conf::WindowMode {
        width: DEFAULT_WINDOW_W,
        height: DEFAULT_WINDOW_H,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        min_height: 0.0,
        max_width: 0.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        resize_on_scale_factor_change: true,
    };

    let path = std::path::PathBuf::from("./src/resources");
    let (mut ctx, event_loop) = ContextBuilder::new("wye_2D", "rust_team")
        .default_conf(c)
        .add_resource_path(path)
        .window_mode(window_mode)
        .build()
        .unwrap();

    graphics::set_window_title(&ctx, "Welcome to Wyeworks!");

    let mut game_state = GameState::new(
        create_batch_sprite(&mut ctx, "/player64.png".to_string()),
        create_batch_sprite(&mut ctx, "/world_atlas.png".to_string()),
    );
    game_state.load_initial_components();
    event::run(ctx, event_loop, game_state);
}

fn create_batch_sprite(ctx: &mut Context, file_name: String) -> SpriteBatch {
    let image = graphics::Image::new(ctx, file_name).unwrap();
    let mut batch = graphics::spritebatch::SpriteBatch::new(image);
    batch.set_filter(graphics::FilterMode::Nearest);
    batch
}
