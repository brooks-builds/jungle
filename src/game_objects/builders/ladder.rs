use ggez::nalgebra::Point2;

use crate::config::Config;
use crate::draw_systems::ladder_draw_system::LadderDrawSystem;
use crate::game_objects::game_object::{GameObjectBuilder, GameObjectBuilderError};
use crate::game_objects::game_object_types::GameObjectfeatureTypes;
use crate::game_objects::GameObject;

pub fn create_ladder(config: &Config) -> Result<GameObject, GameObjectBuilderError> {
    let height = config.cave_height + config.ground_height + config.surface_bottom_height;
    GameObjectBuilder::new()
        .with_type(crate::game_objects::GameObjectTypes::Feature)
        .with_feature_type(GameObjectfeatureTypes::Ladder)
        .height(height)
        .location(Point2::new(
            config.resolution_x / 2.0,
            config.resolution_y - height / 2.0 - config.bedrock_height,
        ))
        .width(config.ladder_width)
        .draw_system(Box::new(LadderDrawSystem::new()))
        .build()
}

#[cfg(test)]
mod test {
    use ggez::nalgebra::Point2;

    use crate::config::Config;
    use crate::game_objects::{GameObject, GameObjectTypes};

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_create_ladder() {
        let config = &Config::default();
        let ladder: GameObject = create_ladder(config).unwrap();
        assert_eq!(ladder.my_type, GameObjectTypes::Feature);
        assert_eq!(ladder.feature_type, Some(GameObjectfeatureTypes::Ladder));
        let expected_height =
            config.cave_height + config.ground_height + config.surface_bottom_height;
        assert_eq!(ladder.height, expected_height);
        let expected_location = Point2::new(
            config.resolution_x / 2.0,
            config.resolution_y - expected_height / 2.0 - config.bedrock_height,
        );
        assert_eq!(ladder.location, expected_location);
        assert_eq!(ladder.width, config.ladder_width);
    }
}
