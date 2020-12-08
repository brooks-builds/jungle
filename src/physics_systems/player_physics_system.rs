use ggez::nalgebra::Point2;

use crate::config::Config;
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
        }
    }

    fn collide_with_ground(&mut self, location: &mut Point2<f32>) {
        if self.on_surface {
            if location.y + self.height / 2.0 > self.surface_floor_y {
                location.y = self.surface_floor_y - self.height / 2.0;
                self.velocity.y = 0.0;
                self.state = PhysicsState::StandingStill;
            }
        } else if location.y + self.height / 2.0 > self.cave_floor_y {
            location.y = self.cave_floor_y - self.height / 2.0;
            self.velocity.y = 0.0;
            self.state = PhysicsState::StandingStill;
        }
    }

    fn handle_command(&mut self, location: &mut Point2<f32>, command: Option<Command>) {
        if let Some(command) = command {
            match command {
                crate::handle_input::Command::Jump => {
                    if location.y + self.height / 2.0 >= self.surface_floor_y {
                        self.velocity.y -= self.jump_force;
                        self.state = PhysicsState::Jumping;
                    }
                }
                crate::handle_input::Command::MoveLeft => {}
                crate::handle_input::Command::MoveRight => {}
                crate::handle_input::Command::StartGame => {}
                crate::handle_input::Command::StopMovingLeft => {}
                crate::handle_input::Command::StopMovingRight => {}
            }
        }
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
        self.collide_with_ground(location);
    }

    fn get_state(&self) -> super::PhysicsState {
        self.state
    }
}

#[cfg(test)]
mod test {
    use ggez::conf::Conf;
    use ggez::nalgebra::Point2;

    use crate::config::Config;
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

    fn create_player_physics_system() -> (PlayerPhysicsSystem, Config) {
        let config = Config::default();
        (PlayerPhysicsSystem::new(&config), config)
    }
}
