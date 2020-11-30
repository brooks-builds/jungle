use std::collections::HashMap;

use ggez::{nalgebra::Point2, Context, GameResult};

use crate::game_objects::GameObjectTypes;
use crate::{
    config::Config, draw_systems::player_draw_system::PlayerDrawSystem, game_objects::GameObject,
    game_objects::GameObjectBuilder, handle_input::Command, images::Images,
    life_systems::player_life_system::PlayerLifeSystem, map::Map,
    physics_systems::player_physics_system::PlayerPhysicsSystem,
};

use super::Scene;

pub struct MainScene {
    map: Map,
    game_objects: HashMap<GameObjectTypes, GameObject>,
}

impl MainScene {
    pub fn new(config: &Config, _context: &mut Context, map: Map) -> GameResult<Self> {
        let mut game_objects = HashMap::new();
        let player = match GameObjectBuilder::new()
            .location(Point2::new(
                config.player_starting_x,
                config.player_starting_y,
            ))
            .width(config.player_width)
            .draw_system(Box::new(PlayerDrawSystem::new(config)))
            .life_system(Box::new(PlayerLifeSystem::new(config.player_lives)))
            .physics_system(Box::new(PlayerPhysicsSystem::new(config)))
            .as_type(GameObjectTypes::Player)
            .build()
        {
            Ok(game_object) => game_object,
            Err(error) => panic!(error),
        };

        game_objects.insert(GameObjectTypes::Player, player);

        // create the heart game object and add it to the game objects vector

        Ok(MainScene { map, game_objects })
    }

    fn is_player_off_screen_right(&self, config: &Config, player: &GameObject) -> bool {
        player.location.x >= config.resolution_x
    }

    fn is_player_off_screen_left(&self, player: &GameObject) -> bool {
        player.location.x + player.width <= 0.0
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
        self.game_objects
            .iter_mut()
            .for_each(|(_, game_object)| game_object.update(command));

        let player = self
            .game_objects
            .get_mut(&GameObjectTypes::Player)
            .expect("Could not find player");

        if player.is_offscreen_right(config.resolution_x) {
            self.map.move_right(&config, context)?;
            player.location.x = 0.0;
        } else if player.is_offscreen_left() {
            self.map.move_left(&config, context)?;
            player.location.x = config.resolution_x - config.player_width;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context, config: &Config, images: &Images) -> GameResult {
        self.map.draw(context, config)?;
        self.game_objects
            .iter_mut()
            .try_for_each(|(_, game_object)| game_object.draw(context, config, images))
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
