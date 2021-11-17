use ggez::*;
pub mod entities {
    pub mod desk;
    pub mod map;
    pub mod player;
}

pub mod positioning;

use entities::map::*;

// The main game state
struct State {
    map: Map,
}

impl State {
    pub fn new() -> Self {
        State { map: Map::new() }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.map.update(ctx)?;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.map.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
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

    let state = State::new();
    event::run(ctx, event_loop, state);
}
