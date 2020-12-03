use ggez::{nalgebra::Point2, Context, GameResult};

use crate::draw_systems::background_draw_system::BackgroundDrawSystem;
use crate::draw_systems::hearts_draw_system::HeartDrawSystem;
use crate::game_objects::{GameObjectBuilderError, GameObjectTypes};
use crate::{
    config::Config, draw_systems::player_draw_system::PlayerDrawSystem, game_objects::GameObject,
    game_objects::GameObjectBuilder, handle_input::Command, images::Images,
    life_systems::player_life_system::PlayerLifeSystem, map::Map,
    physics_systems::player_physics_system::PlayerPhysicsSystem,
};

use super::Scene;

pub struct MainScene {
    map: Map,
    game_objects: Vec<GameObject>,
}

impl MainScene {
    pub fn new(
        config: &Config,
        _context: &mut Context,
        map: Map,
        images: &mut Images,
    ) -> GameResult<Self> {
        let mut game_objects = Vec::new();
        let player = Self::create_player(config).expect("error building player");
        let hearts = Self::create_hearts(config, images).expect("error building hearts");
        let background =
            Self::create_background(config, images).expect("error building background");

        game_objects.push(hearts);
        game_objects.push(background);
        game_objects.push(player);

        Ok(MainScene { map, game_objects })
    }

    fn create_player(config: &Config) -> Result<GameObject, GameObjectBuilderError> {
        GameObjectBuilder::new()
            .location(Point2::new(
                config.player_starting_x,
                config.player_starting_y,
            ))
            .width(config.player_width)
            .draw_system(Box::new(PlayerDrawSystem::new(config)))
            .life_system(Box::new(PlayerLifeSystem::new(config.player_lives)))
            .physics_system(Box::new(PlayerPhysicsSystem::new(config)))
            .with_type(GameObjectTypes::Player)
            .build()
    }

    fn create_hearts(
        config: &Config,
        images: &Images,
    ) -> Result<GameObject, GameObjectBuilderError> {
        GameObjectBuilder::new()
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
            .with_type(GameObjectTypes::Heart)
            .build()
    }

    fn create_background(
        config: &Config,
        images: &Images,
    ) -> Result<GameObject, GameObjectBuilderError> {
        GameObjectBuilder::new()
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
            .with_type(GameObjectTypes::Background)
            .build()
    }

    fn get_first_game_object_by_type(
        &mut self,
        game_object_type: GameObjectTypes,
    ) -> Option<&mut GameObject> {
        self.game_objects
            .iter_mut()
            .find(|game_object| game_object.my_type == game_object_type)
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
            .for_each(|game_object| game_object.update(command));

        let mut move_direction = None;
        if let Some(player) = self.get_first_game_object_by_type(GameObjectTypes::Player) {
            if player.is_offscreen_right(config.resolution_x) {
                move_direction = Some("right");
                player.location.x = 0.0;
            } else if player.is_offscreen_left() {
                move_direction = Some("left");
                player.location.x = config.resolution_x - config.player_width;
            }
        }

        match move_direction {
            Some("right") => self.map.move_right(config, context)?,
            Some("left") => self.map.move_left(config, context)?,
            Some(_) => (),
            None => (),
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context, config: &Config, images: &mut Images) -> GameResult {
        self.map.draw(context, config)?;
        self.game_objects
            .iter_mut()
            .try_for_each(|game_object| game_object.draw(context, config, images))
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
        let main_scene: MainScene = MainScene::new(&config, context, map, &mut images).unwrap();

        assert_eq!(main_scene.game_objects.len(), 3);
    }

    #[test]
    fn test_create_game_objects_in_main_scene() {
        let config = crate::config::load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let map = Map::new(&config, context).unwrap();
        let mut images = Images::new(context, &config).unwrap();
        let _main_scene: MainScene = MainScene::new(&config, context, map, &mut images).unwrap();

        let _player: GameObject = MainScene::create_player(&config).unwrap();
        let _hearts: GameObject = MainScene::create_hearts(&config, &images).unwrap();
        let _background: GameObject = MainScene::create_background(&config, &images).unwrap();
    }

    #[test]
    fn ci_test_get_first_game_object_by_type() {
        let config = crate::config::load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let map = Map::new(&config, context).unwrap();
        let mut images = Images::new(context, &config).unwrap();
        let mut main_scene: MainScene = MainScene::new(&config, context, map, &mut images).unwrap();

        let _player: Option<&mut GameObject> =
            main_scene.get_first_game_object_by_type(GameObjectTypes::Player);
    }
}
