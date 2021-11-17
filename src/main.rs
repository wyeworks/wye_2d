use ggez::*;
pub mod entities {
    pub mod desk;
    pub mod map;
    pub mod player;
}

use entities::map::*;

// Position of the elements in the screen, this will be used by all entities
pub struct Position {
    x: f32,
    y: f32,
}

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

// We use clamping to limit position to a given area. Clamping merely moves the point to the nearest available value
fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}