use ggez::{event::Button, Context, GameResult};

use crate::{config::Config, game_objects::player::Player, map::Map};

use super::Scene;

pub struct MainScene {
    map: Map,
    player: Player,
}

impl MainScene {
    pub fn new(config: &Config, context: &mut Context, map: Map) -> GameResult<Self> {
        let player = Player::new(config, context)?;

        Ok(MainScene { map, player })
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
        self.player.draw(context)?;

        Ok(())
    }
}
