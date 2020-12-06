use ggez::nalgebra::Point2;

use crate::config::Config;
use crate::draw_systems::background_draw_system::BackgroundDrawSystem;
use crate::game_objects::game_object::{GameObjectBuilder, GameObjectBuilderError};
use crate::game_objects::{GameObject, GameObjectTypes};
use crate::images::Images;

pub fn create_background(
    images: &Images,
    config: &Config,
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
        let background: GameObject = super::create_background(images, config).unwrap();

        assert_eq!(background.location.x, 0.0);
        assert_eq!(background.location.y, 0.0);
        assert_eq!(background.my_type, GameObjectTypes::Background);
    }
}
