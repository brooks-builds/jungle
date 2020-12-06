use ggez::nalgebra::Point2;

use crate::config::Config;
use crate::draw_systems::player_draw_system::PlayerDrawSystem;
use crate::game_objects::game_object::{GameObjectBuilder, GameObjectBuilderError};
use crate::game_objects::{GameObject, GameObjectTypes};
use crate::life_systems::player_life_system::PlayerLifeSystem;
use crate::physics_systems::player_physics_system::PlayerPhysicsSystem;

pub fn create_player(config: &Config) -> Result<GameObject, GameObjectBuilderError> {
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

#[cfg(test)]
mod tests {

    use crate::config;
    use crate::game_objects::{GameObject, GameObjectTypes};

    use super::create_player;

    #[test]
    #[allow(clippy::clippy::float_cmp)]
    fn ci_test_create_player_game_object() {
        let config = config::load("config.json").unwrap();
        let player: GameObject = create_player(&config).unwrap();

        assert_eq!(player.location.x, config.player_starting_x);
        assert_eq!(player.location.y, config.player_starting_y);
        assert_eq!(player.my_type, GameObjectTypes::Player);
        assert_eq!(player.width, config.player_width);
    }
}
