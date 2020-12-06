use ggez::{nalgebra::Point2, Context, GameResult};

use crate::config::MapFeature;
use crate::draw_systems::background_draw_system::BackgroundDrawSystem;
use crate::draw_systems::hearts_draw_system::HeartDrawSystem;
use crate::draw_systems::single_pit_draw_system::SinglePitDrawSystem;
use crate::game_objects::builders::background::create_background;
use crate::game_objects::builders::hearts::create_hearts;
use crate::game_objects::builders::player::create_player;
use crate::game_objects::game_object::{GameObjectBuilder, GameObjectBuilderError};
use crate::game_objects::{GameObjectTypes, GameObjects};
use crate::{
    config::Config, draw_systems::player_draw_system::PlayerDrawSystem, game_objects::GameObject,
    handle_input::Command, images::Images, life_systems::player_life_system::PlayerLifeSystem,
    physics_systems::player_physics_system::PlayerPhysicsSystem,
};

pub struct MainScene {
    game_objects: GameObjects,
    current_screen: usize,
}

impl MainScene {
    pub fn new(config: &Config, _context: &mut Context, images: &mut Images) -> GameResult<Self> {
        let mut game_objects = GameObjects::new();
        let player = create_player(config).expect("error creating player");
        let hearts = create_hearts(images, config).expect("error building hearts");
        let background = create_background(images, config).expect("error building background");

        game_objects.push(background);
        game_objects.push(hearts);
        game_objects.push(player);

        let mut main_scene = MainScene {
            game_objects,
            current_screen: config.start_index,
        };

        main_scene.change_screen(config);

        Ok(main_scene)
    }

    fn create_pit1(config: &Config) -> Result<GameObject, GameObjectBuilderError> {
        GameObjectBuilder::new()
            .location(Point2::new(
                config.resolution_x / 2.0,
                config.resolution_y
                    - config.cave_height
                    - config.bedrock_height
                    - config.ground_height
                    - config.surface_height / 2.0,
            ))
            .width(config.pit_width)
            .draw_system(Box::new(SinglePitDrawSystem::new()))
            .with_type(GameObjectTypes::Feature)
            .build()
    }

    pub fn update(
        &mut self,
        command: Option<Command>,
        config: &Config,
        images: &mut Images,
        context: &mut Context,
    ) -> GameResult {
        self.game_objects.update(command);

        if let Some(player) = self.game_objects.get_first_by_type(GameObjectTypes::Player) {
            if player.is_offscreen_right(config.resolution_x) {
                self.current_screen += 1;
                player.location.x = 0.0;
                self.change_screen(config);
                images.reset_trees(context, config)?;
            } else if player.is_offscreen_left() {
                self.current_screen -= 1;
                player.location.x = config.resolution_x;
                self.change_screen(config);
                images.reset_trees(context, config)?;
            }
        }

        Ok(())
    }

    pub fn draw(
        &mut self,
        context: &mut Context,
        config: &Config,
        images: &mut Images,
    ) -> GameResult {
        self.game_objects.draw(context, config, images)
    }

    fn change_screen(&mut self, config: &Config) {
        let player = self.game_objects.remove_player().unwrap();
        self.game_objects.remove_features();
        config.map[self.current_screen]
            .iter()
            .for_each(|map_feature| match map_feature {
                MapFeature::Pit1 => {
                    let pit1 = Self::create_pit1(config).expect("error creating pit1");
                    self.game_objects.push(pit1);
                }
                MapFeature::Pit3 => {}
                MapFeature::Rope => {}
            });
        self.game_objects.push(player);
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
        let mut images = Images::new(context, &config).unwrap();
        let main_scene: MainScene = MainScene::new(&config, context, &mut images).unwrap();

        assert_eq!(main_scene.current_screen, config.start_index);
    }
}
