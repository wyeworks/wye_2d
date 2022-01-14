#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use ecs::{
    constants::{DEFAULT_WINDOW_H, DEFAULT_WINDOW_W},
    game_state::GameState,
};
use ggez::conf::FullscreenType;
use ggez::*;

pub mod ecs {
    pub mod constants;
    pub mod game_state;
    pub mod npcs_loader;
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
    let (ctx, event_loop) = ContextBuilder::new("wye_2D", "rust_team")
        .default_conf(c)
        .window_mode(window_mode)
        .build()
        .unwrap();

    graphics::set_window_title(&ctx, "Welcome to Wyeworks!");

    let mut game_state = GameState::new();
    game_state.load_initial_components();
    event::run(ctx, event_loop, game_state);
}
