use super::ecs::Ecs;
use ggez::event::*;
use ggez::*;

pub struct GameState {
    ecs: Ecs,
}

impl GameState {
    pub fn new() -> Self {
        let mut ecs = Ecs::new();
        ecs.load_initial_components();
        GameState { ecs }
    }
}

impl ggez::event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.ecs.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(130, 90, 44));
        self.ecs.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match key {
            KeyCode::Space => {
                self.ecs = Ecs::new();
                self.ecs.load_initial_components();
            }
            KeyCode::Return => {
                self.ecs.begin_interaction();
            }
            KeyCode::Escape => {
                self.ecs.end_interaction();
            }
            _ => (),
        }
    }
}
