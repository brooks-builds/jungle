use ggez::graphics::Image;
use ggez::nalgebra::Point2;

use crate::config::Config;
use crate::draw_systems::ground_draw_system::GroundDrawSystem;
use crate::draw_systems::tree_draw_system::TreeDrawSystem;
use crate::game_objects::game_object::{GameObjectBuilder, GameObjectBuilderError};
use crate::game_objects::{GameObject, GameObjectTypes};
use crate::images::Images;

pub fn create_behind_ground(
    images: &Images,
    config: &Config,
) -> Result<GameObject, GameObjectBuilderError> {
    GameObjectBuilder::new()
        .location(Point2::new(0.0, 0.0))
        .width(config.resolution_x)
        .draw_system(Box::new(
            GroundDrawSystem::new(images.bedrock.clone())
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
                            - config.ground_height
                            - config.surface_bottom_height
                            - config.surface_top_height,
                    ),
                    config.surface_color,
                    config.surface_top_height / config.bedrock_height,
                )
                .ground(
                    Point2::new(0.0, 0.0),
                    config.sky_color,
                    (config.resolution_y
                        - config.bedrock_height
                        - config.cave_height
                        - config.ground_height
                        - config.surface_bottom_height
                        - config.surface_top_height)
                        / config.bedrock_height,
                ),
        ))
        .with_type(GameObjectTypes::Background)
        .build()
}

pub fn create_above_ground(
    bedrock_image: Image,
    config: &Config,
) -> Result<GameObject, GameObjectBuilderError> {
    let draw_system = GroundDrawSystem::new(bedrock_image)
        .ground(
            Point2::new(
                0.0,
                config.resolution_y
                    - config.bedrock_height
                    - config.cave_height
                    - config.ground_height
                    - config.surface_bottom_height,
            ),
            config.surface_color,
            config.surface_bottom_height / config.bedrock_height,
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
        );
    GameObjectBuilder::new()
        .draw_system(Box::new(draw_system))
        .with_type(GameObjectTypes::Background)
        .height(config.surface_bottom_height + config.ground_height)
        .location(Point2::new(
            0.0,
            config.resolution_y
                - config.bedrock_height
                - config.cave_height
                - config.ground_height
                - config.surface_bottom_height,
        ))
        .width(config.resolution_x)
        .build()
}

pub fn create_trees() -> Result<GameObject, GameObjectBuilderError> {
    GameObjectBuilder::new()
        .with_type(GameObjectTypes::Background)
        .draw_system(Box::new(TreeDrawSystem::new()))
        .build()
}

#[cfg(test)]
mod tests {
    use crate::config;
    use crate::game_objects::{GameObject, GameObjectTypes};
    use crate::images::Images;
    use crate::initialize::initialize;

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_create_background() {
        let config = &config::load("config.json").unwrap();
        let (context, _) = &mut initialize(config).unwrap();
        let images = &Images::new(context, config).unwrap();
        let background: GameObject = super::create_behind_ground(images, config).unwrap();

        assert_eq!(background.location.x, 0.0);
        assert_eq!(background.location.y, 0.0);
        assert_eq!(background.my_type, GameObjectTypes::Background);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_create_above_background() {
        let config = &config::load("config.json").unwrap();
        let (context, _) = &mut initialize(config).unwrap();
        let images = &Images::new(context, config).unwrap();
        let background: GameObject =
            super::create_above_ground(images.bedrock.clone(), config).unwrap();

        assert_eq!(background.feature_type, None);
        assert_eq!(
            background.height,
            config.surface_bottom_height + config.ground_height
        );
        assert_eq!(background.location.x, 0.0);
        assert_eq!(
            background.location.y,
            config.resolution_y
                - config.bedrock_height
                - config.cave_height
                - config.ground_height
                - config.surface_bottom_height
        );
        assert_eq!(background.my_type, GameObjectTypes::Background);
        assert_eq!(background.width, config.resolution_x);
    }

    #[test]
    fn ci_test_create_trees() {
        let trees: GameObject = super::create_trees().unwrap();

        assert_eq!(trees.feature_type, None);
        assert_eq!(trees.my_type, GameObjectTypes::Background);
    }
}
