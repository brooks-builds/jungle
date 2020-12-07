use ggez::nalgebra::Point2;

use crate::config::Config;
use crate::draw_systems::single_pit_draw_system::SinglePitDrawSystem;
use crate::game_objects::game_object::{GameObjectBuilder, GameObjectBuilderError};
use crate::game_objects::game_object_types::GameObjectfeatureTypes;
use crate::game_objects::{GameObject, GameObjectTypes};

pub fn create_pit1(config: &Config) -> Result<GameObject, GameObjectBuilderError> {
    GameObjectBuilder::new()
        .with_type(GameObjectTypes::Feature)
        .location(Point2::new(
            config.resolution_x / 2.0,
            config.resolution_y
                - config.bedrock_height
                - config.cave_height
                - config.ground_height
                - (config.surface_bottom_height + config.surface_top_height) / 2.0,
        ))
        .width(config.pit_width)
        .height(config.pit_height)
        .draw_system(Box::new(SinglePitDrawSystem::new()))
        .with_feature_type(GameObjectfeatureTypes::Pit1)
        .build()
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test {
    use crate::config;
    use crate::game_objects::game_object_types::GameObjectfeatureTypes;
    use crate::game_objects::{GameObject, GameObjectTypes};

    use super::*;

    #[test]
    fn ci_test_create_pit1() {
        let config = &config::load("config.json").unwrap();
        let pit1: GameObject = create_pit1(config).unwrap();

        assert_eq!(pit1.location.x, config.resolution_x / 2.0);
        assert_eq!(
            pit1.location.y,
            config.resolution_y
                - config.bedrock_height
                - config.cave_height
                - config.ground_height
                - (config.surface_bottom_height + config.surface_top_height) / 2.0
        );
        assert_eq!(pit1.my_type, GameObjectTypes::Feature);
        assert_eq!(pit1.width, config.pit_width);
        assert_eq!(pit1.height, config.pit_height);
        assert_eq!(pit1.feature_type, Some(GameObjectfeatureTypes::Pit1));
    }
}
