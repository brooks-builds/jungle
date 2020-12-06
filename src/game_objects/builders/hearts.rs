use ggez::nalgebra::Point2;

use crate::config::Config;
use crate::draw_systems::hearts_draw_system::HeartDrawSystem;
use crate::game_objects::game_object::{GameObjectBuilder, GameObjectBuilderError};
use crate::game_objects::GameObject;
use crate::images::Images;

pub fn create_hearts(
    images: &Images,
    config: &Config,
) -> Result<GameObject, GameObjectBuilderError> {
    GameObjectBuilder::new()
        .draw_system(Box::new(HeartDrawSystem::new(images.life.clone())))
        .location(Point2::new(config.resolution_x - config.life_width, 0.0))
        .width(config.life_width)
        .with_type(crate::game_objects::GameObjectTypes::Heart)
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
    fn test_create_hearts_game_object() {
        let config = &config::load("config.json").unwrap();
        let (context, _) = &mut initialize(config).unwrap();
        let images = &Images::new(context, config).unwrap();
        let hearts: GameObject = super::create_hearts(images, config).unwrap();

        assert_eq!(hearts.location.x, config.resolution_x - config.life_width);
        assert_eq!(hearts.location.y, 0.0);
        assert_eq!(hearts.my_type, GameObjectTypes::Heart);
        assert_eq!(hearts.width, config.life_width);
    }
}
