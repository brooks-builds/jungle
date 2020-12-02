use std::collections::HashMap;

use ggez::nalgebra::Point;
use ggez::{nalgebra::Point2, Context, GameResult};

use crate::draw_systems::background_draw_system::BackgroundDrawSystem;
use crate::draw_systems::hearts_draw_system::HeartDrawSystem;
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
    pub fn new(
        config: &Config,
        _context: &mut Context,
        map: Map,
        images: &mut Images,
    ) -> GameResult<Self> {
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
            .build()
        {
            Ok(game_object) => game_object,
            Err(error) => panic!(error),
        };
        let hearts = match GameObjectBuilder::new()
            .location(Point2::new(
                config.resolution_x - config.life_width * 3.0,
                0.0,
            ))
            .width(config.life_width)
            .draw_system(Box::new(
                HeartDrawSystem::new(images.life.clone())
                    .set_lives(config.player_lives)
                    .set_location(
                        config.resolution_x - config.life_width * config.player_lives as f32,
                        0.0,
                    )
                    .set_width(config.life_width)
                    .build(),
            ))
            .build()
        {
            Ok(heart) => heart,
            Err(error) => panic!(error),
        };
        let background = match GameObjectBuilder::new()
            .location(Point2::new(
                0.0,
                config.resolution_y - config.bedrock_height,
            ))
            .width(config.resolution_x)
            .draw_system(Box::new(
                BackgroundDrawSystem::new(images.bedrock.clone())
                    .bedrock(
                        Point2::new(0.0, config.resolution_y - config.bedrock_height),
                        config.bedrock_color,
                    )
                    .ground(
                        Point2::new(
                            0.0,
                            config.resolution_y
                                - config.bedrock_height
                                - config.cave_height
                                - config.ground_height,
                        ),
                        config.ground_color,
                        config.ground_height / config.bedrock_height,
                    )
                    .ground(
                        Point2::new(
                            0.0,
                            config.resolution_y
                                - config.bedrock_height
                                - config.cave_height
                                - config.ground_height
                                - config.surface_height,
                        ),
                        config.surface_color,
                        config.surface_height / config.bedrock_height,
                    )
                    .ground(
                        Point2::new(0.0, 0.0),
                        config.sky_color,
                        (config.resolution_y
                            - config.bedrock_height
                            - config.cave_height
                            - config.ground_height
                            - config.surface_height)
                            / config.bedrock_height,
                    ),
            ))
            .build()
        {
            Ok(background) => background,
            Err(error) => panic!(error),
        };

        game_objects.insert(GameObjectTypes::Player, player);
        game_objects.insert(GameObjectTypes::Heart, hearts);
        game_objects.insert(GameObjectTypes::Background, background);

        Ok(MainScene { map, game_objects })
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

    fn draw(&mut self, context: &mut Context, config: &Config, images: &mut Images) -> GameResult {
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
        let mut images = Images::new(context, &config).unwrap();
        let mut main_scene: MainScene = MainScene::new(&config, context, map, &mut images).unwrap();
        let mut images = Images::new(context, &config).unwrap();

        main_scene.draw(context, &config, &mut images).unwrap();
    }
}
