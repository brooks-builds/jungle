use ggez::{nalgebra::Point2, Context, GameResult};

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
            config.player_width,
        )?;

        Ok(MainScene { map, player })
    }

    fn is_player_off_screen_right(&self, config: &Config) -> bool {
        self.player.location.x >= config.resolution_x
    }

    fn is_player_off_screen_left(&self) -> bool {
        self.player.location.x + self.player.width <= 0.0
    }
}

impl Scene for MainScene {
    fn update(
        &mut self,
        context: &mut Context,
        command: Option<Command>,
        config: &Config,
        _active_scene: &mut super::ActiveScene,
    ) -> GameResult {
        self.player.update(command);

        if self.is_player_off_screen_right(config) {
            self.map.move_right(&config, context)?;
            self.player.location.x = 0.0;
        } else if self.is_player_off_screen_left() {
            self.map.move_left(&config, context)?;
            self.player.location.x = config.resolution_x - config.player_width;
        }

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
    use crate::{config::load, images::Images, initialize};

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

    #[test]
    fn test_player_moving_off_screen_right() {
        let config = load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let map = Map::new(&config, context).unwrap();
        let mut main_scene = MainScene::new(&config, context, map).unwrap();
        main_scene.player.location.x = config.resolution_x;

        assert_eq!(main_scene.is_player_off_screen_right(&config), true);
    }

    #[test]
    fn test_player_moving_off_screen_left() {
        let config = load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let map = Map::new(&config, context).unwrap();
        let mut main_scene = MainScene::new(&config, context, map).unwrap();
        main_scene.player.location.x = -config.player_width;

        assert_eq!(main_scene.is_player_off_screen_left(), true);
    }

    #[test]
    fn test_player_not_moving_off_screen_right() {
        let config = load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let map = Map::new(&config, context).unwrap();
        let main_scene = MainScene::new(&config, context, map).unwrap();

        assert_eq!(main_scene.is_player_off_screen_right(&config), false);
    }

    #[test]
    fn test_player_not_moving_off_screen_left() {
        let config = load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let map = Map::new(&config, context).unwrap();
        let main_scene = MainScene::new(&config, context, map).unwrap();

        assert_eq!(main_scene.is_player_off_screen_left(), false);
    }
}
