use ggez::nalgebra::Point2;

use crate::config::Config;
use crate::game_objects::GameObject;
use crate::handle_input::Command;

use super::{PhysicsState, PhysicsSystem};

pub struct PlayerPhysicsSystem {
    state: PhysicsState,
    gravity_force: f32,
    velocity: Point2<f32>,
    surface_floor_y: f32,
    height: f32,
    jump_force: f32,
    on_surface: bool,
    cave_floor_y: f32,
    speed: f32,
    width: f32,
}

impl PlayerPhysicsSystem {
    pub fn new(config: &Config) -> Self {
        Self {
            state: PhysicsState::StandingStill,
            gravity_force: config.gravity_force,
            velocity: Point2::new(0.0, 0.0),
            surface_floor_y: config.surface_floor_y,
            height: config.player_height,
            jump_force: config.jump_force,
            on_surface: true,
            cave_floor_y: config.resolution_y - config.bedrock_height,
            speed: config.player_speed,
            width: config.player_width,
        }
    }

    fn collide_with_ground(&mut self, location: &mut Point2<f32>, features: Vec<GameObject>) {
        // if we collide with a pit, then we reset floor to the cave floor
        let floor_y = if self.on_surface {
            self.surface_floor_y
        } else {
            self.cave_floor_y
        };

        features.iter().for_each(|feature| {
            if let Some(feature_type) = feature.feature_type {
                match feature_type {
                    crate::game_objects::game_object_types::GameObjectfeatureTypes::Pit1 => {
                        if self.is_colliding_with(location, feature)
                            && location.y + self.height / 2.0 > floor_y
                            && self.on_surface
                        {
                            self.on_surface = false;
                        }
                    }
                }
            }
        });

        if location.y + self.height / 2.0 > floor_y {
            location.y = floor_y - self.height / 2.0;
            self.velocity.y = 0.0;
            self.state = if self.state == PhysicsState::Jumping {
                PhysicsState::StandingStill
            } else {
                self.state
            };
        }
    }

    fn is_colliding_with(&self, location: &mut Point2<f32>, other: &GameObject) -> bool {
        location.x - self.width / 2.0 > other.location.x - other.width / 2.0
            && location.x + self.width / 2.0 < other.location.x + other.width / 2.0
            && location.y + self.height / 2.0 > other.location.y - other.height / 2.0
            && location.y - self.height / 2.0 < other.location.y + other.height / 2.0
    }

    fn handle_command(&mut self, location: &mut Point2<f32>, command: Option<Command>) {
        if let Some(command) = command {
            match command {
                crate::handle_input::Command::Jump => self.handle_jump_command(location),
                crate::handle_input::Command::MoveLeft => self.handle_move_left_command(),
                crate::handle_input::Command::MoveRight => self.handle_move_right_command(),
                crate::handle_input::Command::StartGame => {}
                crate::handle_input::Command::StopMovingLeft => {
                    self.handle_stop_moving_left_command()
                }
                crate::handle_input::Command::StopMovingRight => {
                    self.handle_stop_moving_right_command()
                }
            }
        }
    }

    fn handle_jump_command(&mut self, location: &mut Point2<f32>) {
        let floor_y = if self.on_surface {
            self.surface_floor_y
        } else {
            self.cave_floor_y
        };
        if location.y + self.height / 2.0 >= floor_y {
            self.velocity.y -= self.jump_force;
            self.state = PhysicsState::Jumping;
        }
    }

    fn handle_move_right_command(&mut self) {
        self.state = PhysicsState::MovingRight;
        self.velocity.x += self.speed;
    }

    fn handle_stop_moving_right_command(&mut self) {
        self.velocity.x = 0.0;
        self.state = PhysicsState::StandingStill;
    }

    fn handle_move_left_command(&mut self) {
        self.state = PhysicsState::MovingLeft;
        self.velocity.x -= self.speed;
    }

    fn handle_stop_moving_left_command(&mut self) {
        self.state = PhysicsState::StandingStill;
        self.velocity.x = 0.0;
    }
}

impl PhysicsSystem for PlayerPhysicsSystem {
    fn update(
        &mut self,
        location: &mut ggez::nalgebra::Point2<f32>,
        command: Option<crate::handle_input::Command>,
        features: Vec<crate::game_objects::GameObject>,
    ) {
        self.velocity.y += self.gravity_force;
        self.handle_command(location, command);
        location.y += self.velocity.y;
        location.x += self.velocity.x;
        self.collide_with_ground(location, features);
    }

    fn get_state(&self) -> super::PhysicsState {
        self.state
    }
}

#[cfg(test)]
mod test {
    use ggez::nalgebra::Point2;

    use crate::config::Config;
    use crate::game_objects::builders::pit1::create_pit1;
    use crate::handle_input::Command;
    use crate::physics_systems::PhysicsState;

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_player_physics_system() {
        let (player_physics_system, config) = create_player_physics_system();
        assert_eq!(player_physics_system.gravity_force, config.gravity_force);
        assert_eq!(player_physics_system.velocity, Point2::new(0.0, 0.0));
        assert_eq!(
            player_physics_system.surface_floor_y,
            config.surface_floor_y
        );
        assert_eq!(player_physics_system.height, config.player_height);
        assert_eq!(player_physics_system.jump_force, config.jump_force);
        assert_eq!(player_physics_system.on_surface, true);
        assert_eq!(
            player_physics_system.cave_floor_y,
            config.resolution_y - config.bedrock_height
        );
        assert_eq!(player_physics_system.speed, config.player_speed);
        assert_eq!(player_physics_system.width, config.player_width);
    }

    #[test]
    fn ci_test_player_physics_system_get_state() {
        let (player_physics_system, _) = create_player_physics_system();
        assert_eq!(
            player_physics_system.get_state(),
            PhysicsState::StandingStill
        );
    }

    #[test]
    fn ci_test_player_physics_system_update() {
        let (mut player_physics_system, config) = create_player_physics_system();
        let mut location = Point2::new(0.0, config.player_starting_y);
        let no_command = None;
        let features = vec![];
        player_physics_system.update(&mut location, no_command, features.clone());
        assert_eq!(location, Point2::new(0.0, config.player_starting_y));
        assert_eq!(player_physics_system.velocity, Point2::new(0.0, 0.0));

        let jump_command = Some(Command::Jump);
        player_physics_system.update(&mut location, jump_command, features.clone());
        assert_eq!(
            player_physics_system.velocity,
            Point2::new(0.0, config.gravity_force - config.jump_force)
        );
        assert_eq!(player_physics_system.state, PhysicsState::Jumping);
        player_physics_system.update(&mut location, no_command, features.clone());
        assert_eq!(player_physics_system.state, PhysicsState::StandingStill);

        player_physics_system.on_surface = false;
        location.y = player_physics_system.cave_floor_y - player_physics_system.height / 2.0;
        player_physics_system.update(&mut location, no_command, features);
        assert_eq!(
            location,
            Point2::new(
                0.0,
                player_physics_system.cave_floor_y - player_physics_system.height / 2.0
            )
        );
    }

    #[test]
    fn ci_test_player_physics_update_move_right() {
        let (mut player_physics_system, config) = create_player_physics_system();
        let mut location = Point2::new(0.0, config.player_starting_y);
        let move_right_command = Some(Command::MoveRight);
        let features = vec![];
        player_physics_system.update(&mut location, move_right_command, features.clone());
        assert_eq!(
            location,
            Point2::new(config.player_speed, config.player_starting_y)
        );
        assert_eq!(player_physics_system.state, PhysicsState::MovingRight);

        let stop_moving_right_command = Some(Command::StopMovingRight);
        player_physics_system.update(&mut location, stop_moving_right_command, features);
        assert_eq!(player_physics_system.state, PhysicsState::StandingStill);
        assert_eq!(
            location,
            Point2::new(config.player_speed, config.player_starting_y)
        );
    }

    #[test]
    fn ci_test_player_physics_update_move_left() {
        let (mut player_physics_system, config) = create_player_physics_system();
        let mut location = Point2::new(0.0, config.player_starting_y);
        let move_left_command = Some(Command::MoveLeft);
        let features = vec![];
        player_physics_system.update(&mut location, move_left_command, features.clone());
        assert_eq!(
            location,
            Point2::new(-config.player_speed, config.player_starting_y)
        );
        assert_eq!(player_physics_system.state, PhysicsState::MovingLeft);

        let stop_moving_left_command = Some(Command::StopMovingLeft);
        player_physics_system.update(&mut location, stop_moving_left_command, features);
        assert_eq!(player_physics_system.state, PhysicsState::StandingStill);
        assert_eq!(
            location,
            Point2::new(-config.player_speed, config.player_starting_y)
        );
    }

    #[test]
    fn ci_test_player_physics_system_falling_into_pit() {
        let (mut player_physics_system, config) = create_player_physics_system();
        let mut location = Point2::new(config.player_starting_x, config.player_starting_y);
        let features = vec![create_pit1(&config).unwrap()];
        let no_command = None;
        player_physics_system.update(&mut location, no_command, features.clone());
        assert_eq!(
            location,
            Point2::new(config.player_starting_x, config.player_starting_y)
        );
        location.x = features[0].location.x;
        player_physics_system.update(&mut location, no_command, features.clone());
        player_physics_system.update(&mut location, no_command, features.clone());
        assert_eq!(
            location,
            Point2::new(
                features[0].location.x,
                config.player_starting_y + player_physics_system.gravity_force
            )
        );
    }

    fn create_player_physics_system() -> (PlayerPhysicsSystem, Config) {
        let config = Config::default();
        (PlayerPhysicsSystem::new(&config), config)
    }
}
