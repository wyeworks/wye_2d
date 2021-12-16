use super::world::World;
use ggez::event::*;
use ggez::*;

pub struct GameState {
    world: World,
}

impl GameState {
    pub fn new() -> Self {
        let mut world = World::new();
        world.load_initial_components();
        GameState { world }
    }
}

impl ggez::event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.world.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.world.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match key {
            KeyCode::Space => {
                self.world = World::new();
            }
            KeyCode::Return => {
                self.world.begin_interaction(ctx);
            }
            _ => (),
        }
    }
}
