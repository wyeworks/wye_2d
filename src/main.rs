#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use ecs::{
    game_state::GameState,
    utils::constants::{DEFAULT_WINDOW_H, DEFAULT_WINDOW_W},
};
use ggez::conf::FullscreenType;
use ggez::*;

pub mod ecs {
    pub mod components {
        pub mod npc;
    }

    pub mod sprites {
        pub mod atlas;
        pub mod player_sprite;
        pub mod tile;
    }

    pub mod systems {
        pub mod input_system {
            pub mod player_input_system;
        }

        pub mod physics_system {
            pub mod physics_system;
            pub mod positioning {
                pub mod collision;
                pub mod positioning;
            }
        }
        pub mod render_system {
            pub mod camera_system;
            pub mod render_system;
        }
    }

    pub mod utils {
        pub mod constants;
        pub mod npcs_loader;
    }

    pub mod game_state;
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

    let game_state = GameState::new(&mut ctx);
    event::run(ctx, event_loop, game_state);
}
