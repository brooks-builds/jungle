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
    jump_force: f32,
    gravity_force: f32,
    surface_floor_y: f32,
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
        let jump_force = -config.jump_force;
        let gravity_force = config.gravity_force;
        let surface_floor_y = config.resolution_y
            - config.bedrock_height
            - config.cave_height
            - config.ground_height
            - config.surface_bottom_height
            - config.player_height / 2.0;

        Self {
            state,
            velocity,
            speed,
            width,
            height,
            cave_floor_y,
            on_surface,
            jump_force,
            gravity_force,
            surface_floor_y,
        }
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Jump => match self.state {
                PhysicsState::Falling => {}
                PhysicsState::Jumping => {}
                PhysicsState::MovingLeft => self.state = PhysicsState::Jumping,
                PhysicsState::MovingRight => self.state = PhysicsState::Jumping,
                PhysicsState::StandingStill => self.state = PhysicsState::Jumping,
            },
            Command::MoveLeft => self.state = PhysicsState::MovingLeft,
            Command::MoveRight => self.state = PhysicsState::MovingRight,
            Command::StartGame => {}
            Command::StopMovingLeft => self.state = PhysicsState::StandingStill,
            Command::StopMovingRight => self.state = PhysicsState::StandingStill,
        }
    }

    fn handle_state(&mut self, location: &mut Point2<f32>) {
        match self.state {
            PhysicsState::Falling => {
                self.apply_gravity();
                if self.on_surface {
                    if location.y > self.surface_floor_y {
                        location.y = self.surface_floor_y;
                        self.state = PhysicsState::StandingStill;
                        self.velocity.y = 0.0;
                    }
                } else if location.y > self.cave_floor_y {
                    location.y = self.cave_floor_y;
                    self.state = PhysicsState::StandingStill;
                    self.velocity.y = 0.0;
                }
            }
            PhysicsState::Jumping => {
                self.velocity.y = self.jump_force;
                self.state = PhysicsState::Falling;
            }
            PhysicsState::MovingLeft => {
                self.velocity.x = -self.speed;
            }
            PhysicsState::MovingRight => {
                self.velocity.x = self.speed;
            }
            PhysicsState::StandingStill => {
                self.velocity.x = 0.0;
            }
        }
    }

    fn apply_gravity(&mut self) {
        self.velocity.y += self.gravity_force;
    }
}

impl PhysicsSystem for PlayerPhysicsSystem {
    fn update(
        &mut self,
        location: &mut Point2<f32>,
        command: Option<crate::handle_input::Command>,
        features: Vec<GameObject>,
    ) {
        if let Some(command) = command {
            self.handle_command(command);
        }

        self.handle_state(location);

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

    use crate::draw_systems::player_draw_system::PlayerDrawSystem;
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

    #[test]
    fn ci_test_set_state_to_jumping() {
        let config = Config::default();
        let mut player_physics_system = PlayerPhysicsSystem::new(&config);
        assert_eq!(player_physics_system.state, PhysicsState::StandingStill);
        let command = Command::Jump;

        player_physics_system.handle_command(command);
        assert_eq!(player_physics_system.state, PhysicsState::Jumping);

        player_physics_system.state = PhysicsState::MovingLeft;
        player_physics_system.handle_command(command);
        assert_eq!(player_physics_system.state, PhysicsState::Jumping);

        player_physics_system.state = PhysicsState::MovingRight;
        player_physics_system.handle_command(command);
        assert_eq!(player_physics_system.state, PhysicsState::Jumping);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_handle_jumping_state() {
        let config = Config::default();
        let mut player_physics_system = PlayerPhysicsSystem::new(&config);
        player_physics_system.state = PhysicsState::Jumping;
        let mut location = Point2::new(0.0, 0.0);

        player_physics_system.handle_state(&mut location);

        assert_eq!(
            player_physics_system.velocity.y,
            player_physics_system.jump_force
        );
        assert_eq!(player_physics_system.state, PhysicsState::Falling);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_apply_gravity() {
        let config = Config::default();
        let mut player_physics_system = PlayerPhysicsSystem::new(&config);
        player_physics_system.state = PhysicsState::Falling;
        player_physics_system.apply_gravity();
        assert_eq!(
            player_physics_system.velocity.y,
            player_physics_system.gravity_force
        );
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_update_stop_falling_when_hit_surface() {
        let config = Config::default();
        let mut player_physics_system = PlayerPhysicsSystem::new(&config);
        let mut location = Point2::new(0.0, player_physics_system.surface_floor_y);
        let command = None;
        let features = vec![];

        player_physics_system.state = PhysicsState::Falling;
        player_physics_system.velocity.y = 1.0;

        player_physics_system.update(&mut location, command, features);
        player_physics_system.update(&mut location, command, vec![]);
        assert_eq!(
            player_physics_system.surface_floor_y,
            config.resolution_y
                - config.bedrock_height
                - config.cave_height
                - config.ground_height
                - config.surface_bottom_height
                - config.player_height / 2.0
        );
        assert_eq!(location.y, player_physics_system.surface_floor_y);
        assert_eq!(player_physics_system.state, PhysicsState::StandingStill);
        assert_eq!(player_physics_system.velocity.y, 0.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_falling_when_updating() {
        let config = Config::default();
        let mut player_physics_system = PlayerPhysicsSystem::new(&config);
        let mut location = Point2::new(0.0, -1.0);
        let command = None;
        let features = vec![];

        player_physics_system.state = PhysicsState::Falling;
        let expected_location_y = location.y + player_physics_system.gravity_force;

        player_physics_system.update(&mut location, command, features);
        assert_eq!(location.x, 0.0);
        assert_eq!(location.y, expected_location_y);
    }
}
