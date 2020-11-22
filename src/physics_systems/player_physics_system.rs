use crate::{config::Config, handle_input::Command};

use super::PhysicsSystem;

#[derive(PartialEq, Debug)]
pub enum PlayerState {
    Standing,
}

pub struct PlayerPhysicsSystem {
    state: PlayerState,
    velocity: f32,
    speed: f32,
}

impl PlayerPhysicsSystem {
    pub fn new(config: &Config) -> Self {
        let state = PlayerState::Standing;
        let velocity = 0.0;
        let speed = config.player_speed;

        Self {
            state,
            velocity,
            speed,
        }
    }
}

impl PhysicsSystem for PlayerPhysicsSystem {
    fn update(
        &mut self,
        location: &mut ggez::nalgebra::Point2<f32>,
        command: Option<crate::handle_input::Command>,
    ) {
        if let Some(command) = command {
            if command == Command::MoveRight {
                self.velocity += self.speed;
            }
        } else {
            self.velocity = 0.0;
        }

        location.x += self.velocity
    }
}

#[cfg(test)]
mod test {
    use ggez::nalgebra::Point2;

    use crate::{config, handle_input::Command};

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_new_player_physics_system() {
        let config = config::load("config.json").unwrap();
        let player_physics_system = PlayerPhysicsSystem::new(&config);
        assert_eq!(player_physics_system.state, PlayerState::Standing);
        assert_eq!(player_physics_system.velocity, 0.0);
        assert_eq!(player_physics_system.speed, config.player_speed);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_update_player_not_moving() {
        let config = config::load("config.json").unwrap();
        let mut player_physics_system = PlayerPhysicsSystem::new(&config);
        let mut location = Point2::new(0.0, 0.0);
        player_physics_system.update(&mut location, None);
        assert_eq!(location.x, 0.0);
        assert_eq!(location.y, 0.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_update_player_moving_right() {
        let config = config::load("config.json").unwrap();
        let mut player_physics_system = PlayerPhysicsSystem::new(&config);
        let mut location = Point2::new(0.0, 0.0);
        let command = Command::MoveRight;
        player_physics_system.update(&mut location, Some(command));
        assert_eq!(location.x, config.player_speed);
        assert_eq!(location.y, 0.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_update_stop_player_moving_right() {
        let config = config::load("config.json").unwrap();
        let mut player_physics_system = PlayerPhysicsSystem::new(&config);
        let mut location = Point2::new(0.0, 0.0);
        let command = Command::MoveRight;
        player_physics_system.update(&mut location, Some(command));
        player_physics_system.update(&mut location, None);
        assert_eq!(location.x, config.player_speed);
        assert_eq!(location.y, 0.0);
    }
}
