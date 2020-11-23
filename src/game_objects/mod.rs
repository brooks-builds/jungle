pub mod bedrock;
pub mod foliage;
pub mod ground;
pub mod pit;
pub mod surface;
pub mod surface_background;
pub mod tree_trunks;

use ggez::{nalgebra::Point2, Context, GameResult};

use crate::{
    config::Config, draw_systems::DrawSystem, handle_input::Command, images::Images,
    physics_systems::PhysicsSystem,
};

pub trait StaticGameObject {
    fn draw(&self, config: &Config, context: &mut Context) -> GameResult;
}

pub struct GameObject {
    location: Point2<f32>,
    draw_system: Box<dyn DrawSystem>,
    physics_system: Option<Box<dyn PhysicsSystem>>,
}

impl GameObject {
    pub fn new(
        location: Point2<f32>,
        draw_system: Box<dyn DrawSystem>,
        physics_system: Option<Box<dyn PhysicsSystem>>,
    ) -> GameResult<Self> {
        Ok(Self {
            location,
            draw_system,
            physics_system,
        })
    }

    pub fn draw(&mut self, context: &mut Context, config: &Config, images: &Images) -> GameResult {
        let physics_state = if let Some(physics_system) = &self.physics_system {
            Some(physics_system.get_state())
        } else {
            None
        };
        self.draw_system
            .draw(images, config, context, &self.location, physics_state)
    }

    pub fn update(&mut self, command: Option<Command>) {
        if let Some(physics_system) = &mut self.physics_system {
            physics_system.update(&mut self.location, command);
        }
    }
}

#[cfg(test)]
mod test {
    use ggez::nalgebra::Point2;

    use crate::{config, physics_systems::player_physics_system::PlayerPhysicsSystem};
    use crate::{draw_systems::player_draw_system::PlayerDrawSystem, images::Images, initialize};

    use super::*;

    #[test]
    fn ci_test_create_empty_game_object() {
        let config = config::load("config.json").unwrap();
        let location: Point2<f32> = Point2::new(10.0, 10.0);
        let player_draw_system = PlayerDrawSystem::new(&config);
        let player_physics_system = PlayerPhysicsSystem::new(&config);
        let game_object: GameObject = GameObject::new(
            location,
            Box::new(player_draw_system),
            Some(Box::new(player_physics_system)),
        )
        .unwrap();
        assert_eq!(game_object.location, location);
    }

    #[test]
    fn test_draw_game_object() {
        let location: Point2<f32> = Point2::new(10.0, 10.0);
        let config = config::load("config.json").unwrap();
        let player_draw_system = PlayerDrawSystem::new(&config);
        let mut game_object: GameObject =
            GameObject::new(location, Box::new(player_draw_system), None).unwrap();
        let config = crate::config::load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let images = Images::new(context, &config).unwrap();
        game_object.draw(context, &config, &images).unwrap();
    }

    #[test]
    fn ci_test_update_game_object() {
        let config = crate::config::load("config.json").unwrap();
        let location: Point2<f32> = Point2::new(10.0, 10.0);
        let player_draw_system = PlayerDrawSystem::new(&config);
        let physics_system = PlayerPhysicsSystem::new(&config);
        let mut player: GameObject = GameObject::new(
            location,
            Box::new(player_draw_system),
            Some(Box::new(physics_system)),
        )
        .unwrap();
        let command = Command::MoveRight;
        player.update(Some(command));
    }
}
