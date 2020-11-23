use ggez::{event::Button, nalgebra::Point2, Context, GameResult};

use crate::{
    config::Config, draw_systems::player_draw_system::PlayerDrawSystem, game_objects::GameObject,
    handle_input::Command, images::Images, map::Map,
    physics_systems::player_physics_system::PlayerPhysicsSystem,
};

use super::Scene;

pub struct MainScene {
    map: Map,
    player: GameObject,
}

impl MainScene {
    pub fn new(config: &Config, _context: &mut Context, map: Map) -> GameResult<Self> {
        let player = GameObject::new(
            Point2::new(config.player_starting_x, config.player_starting_y),
            Box::new(PlayerDrawSystem::new(config)),
            Some(Box::new(PlayerPhysicsSystem::new(config))),
        )?;

        Ok(MainScene { map, player })
    }
}

impl Scene for MainScene {
    fn update(
        &mut self,
        _context: &mut Context,
        command: Option<Command>,
        _config: &Config,
        _active_scene: &mut super::ActiveScene,
    ) -> GameResult {
        self.player.update(command);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context, config: &Config, images: &Images) -> GameResult {
        self.map.draw(context, config)?;
        self.player.draw(context, config, images)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{images::Images, initialize};

    use super::*;

    #[test]
    fn test_create_main_scene() {
        let config = crate::config::load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let map = Map::new(&config, context).unwrap();
        let mut main_scene: MainScene = MainScene::new(&config, context, map).unwrap();
        let images = Images::new(context, &config).unwrap();

        main_scene.draw(context, &config, &images).unwrap();
    }
}
