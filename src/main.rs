use ecs::game_state::GameState;
use ggez::*;

pub mod ecs {
    pub mod constants;
    pub mod ecs;
    pub mod game_state;
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
        pub mod render_system;
        pub mod player_input_system;
    }
}

fn main() -> GameResult {
    let c = conf::Conf::new();

    let (ctx, event_loop) = ContextBuilder::new("wye_2D", "rust_team")
        .default_conf(c)
        .window_mode(ggez::conf::WindowMode::default().dimensions(1300.0, 800.0))
        .build()
        .unwrap();

    graphics::set_window_title(&ctx, "Welcome to Wyeworks!");

    let state = GameState::new();
    event::run(ctx, event_loop, state);
}
