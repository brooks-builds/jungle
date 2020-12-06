use ggez::nalgebra::Point2;

use crate::game_objects::game_object_types::GameObjectfeatureTypes;
use crate::game_objects::GameObject;
use crate::{config::Config, handle_input::Command};

use super::{PhysicsState, PhysicsSystem};

pub struct PlayerPhysicsSystem {
    state: PhysicsState,
    velocity: Point2<f32>,
    speed: f32,
    width: f32,
    height: f32,
    cave_floor_y: f32,
    on_surface: bool,
}

impl PlayerPhysicsSystem {
    pub fn new(config: &Config) -> Self {
        let state = PhysicsState::StandingStill;
        let velocity = Point2::new(0.0, 0.0);
        let speed = config.player_speed;
        let width = config.player_width;
        let height = config.player_height;
        let cave_floor_y = config.resolution_y - config.bedrock_height;
        let on_surface = true;

        Self {
            state,
            velocity,
            speed,
            width,
            height,
            cave_floor_y,
            on_surface,
        }
    }
}

impl PhysicsSystem for PlayerPhysicsSystem {
    fn update(
        &mut self,
        location: &mut Point2<f32>,
        command: Option<crate::handle_input::Command>,
        features: Vec<GameObject>,
    ) {
        if self.state != PhysicsState::Falling {
            if let Some(command) = command {
                match command {
                    Command::MoveRight => self.state = PhysicsState::MovingRight,
                    Command::StopMovingRight => self.state = PhysicsState::StandingStill,
                    Command::StartGame => {}
                    Command::MoveLeft => self.state = PhysicsState::MovingLeft,
                    Command::StopMovingLeft => self.state = PhysicsState::StandingStill,
                }
            }
        }

        match self.state {
            PhysicsState::MovingRight => self.velocity.x = self.speed,
            PhysicsState::StandingStill => {
                self.velocity.x = 0.0;
                self.velocity.y = 0.0;
            }
            PhysicsState::MovingLeft => self.velocity.x = -self.speed,
            PhysicsState::Falling => {
                self.velocity.x = 0.0;
                self.velocity.y = self.speed;
                if location.y + self.height / 2.0 >= self.cave_floor_y {
                    location.y = self.cave_floor_y - self.height / 2.0;
                    self.state = PhysicsState::StandingStill;
                }
            }
        }
        // dbg!(self.state);

        features.iter().for_each(|feature| {
            if let Some(feature_type) = feature.feature_type {
                match feature_type {
                    GameObjectfeatureTypes::Pit1 => {
                        if self.on_surface
                            && (location.x - self.width / 2.0
                                >= feature.location.x - feature.width / 2.0
                                && location.x + self.width / 2.0
                                    <= feature.location.x + feature.width / 2.0)
                        {
                            self.state = PhysicsState::Falling;
                            self.on_surface = false;
                        }
                    }
                }
            }
        });

        location.x += self.velocity.x;
        location.y += self.velocity.y;
    }

    fn get_state(&self) -> super::PhysicsState {
        self.state
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
        assert_eq!(player_physics_system.state, PhysicsState::StandingStill);
        assert_eq!(player_physics_system.velocity.x, 0.0);
        assert_eq!(player_physics_system.velocity.y, 0.0);
        assert_eq!(player_physics_system.speed, config.player_speed);
        assert_eq!(player_physics_system.width, config.player_width);
        assert_eq!(player_physics_system.height, config.player_height);
        assert_eq!(
            player_physics_system.cave_floor_y,
            config.resolution_y - config.bedrock_height
        );
        assert_eq!(player_physics_system.on_surface, true);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_update_player_not_moving() {
        let config = config::load("config.json").unwrap();
        let mut player_physics_system = PlayerPhysicsSystem::new(&config);
        let mut location = Point2::new(0.0, 0.0);
        player_physics_system.update(&mut location, None, vec![]);
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
        player_physics_system.update(&mut location, Some(command), vec![]);
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
        player_physics_system.update(&mut location, Some(command), vec![]);
        assert_eq!(location.x, config.player_speed);
        assert_eq!(location.y, 0.0);
        assert_eq!(player_physics_system.state, PhysicsState::MovingRight);
        player_physics_system.update(&mut location, None, vec![]);
        assert_eq!(location.x, config.player_speed * 2.0);
        assert_eq!(location.y, 0.0);
        player_physics_system.update(&mut location, Some(Command::StopMovingRight), vec![]);
        assert_eq!(location.x, config.player_speed * 2.0);
        assert_eq!(location.y, 0.0);
        assert_eq!(player_physics_system.state, PhysicsState::StandingStill);
    }

    #[test]
    #[allow(clippy::clippy::float_cmp)]
    fn ci_test_player_moves_left() {
        let config = config::load("config.json").unwrap();
        let mut player_physics_system = PlayerPhysicsSystem::new(&config);
        let mut location = Point2::new(0.0, 0.0);
        player_physics_system.update(&mut location, Some(Command::MoveLeft), vec![]);
        assert_eq!(location.x, -config.player_speed);
        assert_eq!(location.y, 0.0);
        assert_eq!(player_physics_system.state, PhysicsState::MovingLeft);
        player_physics_system.update(&mut location, None, vec![]);
        assert_eq!(location.x, -config.player_speed * 2.0);
        assert_eq!(location.y, 0.0);
        assert_eq!(player_physics_system.state, PhysicsState::MovingLeft);
        player_physics_system.update(&mut location, Some(Command::StopMovingLeft), vec![]);
        assert_eq!(location.x, -config.player_speed * 2.0);
        assert_eq!(location.y, 0.0);
        assert_eq!(player_physics_system.state, PhysicsState::StandingStill);
    }
}
