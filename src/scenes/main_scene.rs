use ggez::{event::Button, Context, GameResult};

use crate::{config::Config, map::Map};

use super::Scene;

pub struct MainScene {
    map: Map,
}

impl MainScene {
    pub fn new(_config: &Config, _context: &mut Context, map: Map) -> GameResult<Self> {
        Ok(MainScene { map })
    }
}

impl Scene for MainScene {
    fn update(
        &mut self,
        _context: &mut Context,
        _button_pressed: Option<Button>,
        _config: &Config,
        _active_scene: &mut super::ActiveScene,
    ) -> GameResult {
        Ok(())
    }

    fn draw(&self, context: &mut Context, config: &Config) -> GameResult {
        self.map.draw(context, config)?;

        Ok(())
    }
}
