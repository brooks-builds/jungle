use ggez::{
    graphics::Rect,
    graphics::{self, DrawParam},
    nalgebra::Point2,
    Context, GameResult,
};

use crate::{
    config::Config,
    images::Images,
    physics_systems::{PhysicsState, PhysicsSystem},
};

use super::DrawSystem;

pub struct PlayerDrawSystem {
    frames_until_sprite_change: u8,
    spritesheet_portion: Rect,
}

impl PlayerDrawSystem {
    pub fn new(config: &Config) -> Self {
        Self {
            frames_until_sprite_change: config.spritesheet_animation_speed,
            spritesheet_portion: Rect::new(
                0.0,
                0.0,
                1.0 / config.player_running_spritesheet_count,
                1.0,
            ),
        }
    }
}

impl DrawSystem for PlayerDrawSystem {
    fn draw(
        &mut self,
        images: &Images,
        config: &Config,
        context: &mut Context,
        location: &Point2<f32>,
        physics_state: Option<PhysicsState>,
    ) -> GameResult {
        if let Some(state) = physics_state {
            match state {
                PhysicsState::StandingStill => graphics::draw(
                    context,
                    &images.standing_player,
                    DrawParam::new().dest([location.x, location.y]),
                ),
                PhysicsState::MovingRight => {
                    self.frames_until_sprite_change = if self.frames_until_sprite_change == 0 {
                        self.spritesheet_portion.x += 1.0 / config.player_running_spritesheet_count;
                        if self.spritesheet_portion.x >= 1.0 {
                            self.spritesheet_portion.x = 0.0;
                        }
                        config.spritesheet_animation_speed
                    } else {
                        self.frames_until_sprite_change - 1
                    };
                    graphics::draw(
                        context,
                        &images.running_player,
                        DrawParam::new()
                            .dest([location.x, location.y])
                            .src(self.spritesheet_portion),
                    )
                }
                PhysicsState::MovingLeft => graphics::draw(
                    context,
                    &images.running_player,
                    DrawParam::new()
                        .dest([location.x, location.y])
                        .src(self.spritesheet_portion)
                        .offset(Point2::new(1.0, 0.0))
                        .scale([-1.0, 1.0]),
                ),
            }
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        config, initialize::initialize, physics_systems::player_physics_system::PlayerPhysicsSystem,
    };

    use super::*;

    #[test]
    fn ci_test_create_player_draw_system() {
        let config = config::load("config.json").unwrap();
        let mut player_draw_system = PlayerDrawSystem::new(&config);

        assert_eq!(
            player_draw_system.frames_until_sprite_change,
            config.spritesheet_animation_speed
        );

        assert_eq!(
            player_draw_system.spritesheet_portion,
            Rect::new(0.0, 0.0, 1.0 / config.player_running_spritesheet_count, 1.0)
        );
    }

    #[test]
    fn test_player_draw_system_frames_until_sprite_change() {
        let mut config = config::load("config.json").unwrap();
        config.spritesheet_animation_speed = 2;
        let mut player_draw_system = PlayerDrawSystem::new(&config);

        assert_eq!(
            player_draw_system.frames_until_sprite_change,
            config.spritesheet_animation_speed
        );
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        assert_eq!(player_draw_system.frames_until_sprite_change, 1);
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        assert_eq!(player_draw_system.frames_until_sprite_change, 2);
    }

    #[test]
    fn test_player_draw_system_spritesheet_portion() {
        let mut config = config::load("config.json").unwrap();
        config.spritesheet_animation_speed = 1;
        let mut player_draw_system = PlayerDrawSystem::new(&config);
        let mut expected_spritesheet_portion =
            Rect::new(0.0, 0.0, 1.0 / config.player_running_spritesheet_count, 1.0);
        assert_eq!(
            player_draw_system.spritesheet_portion,
            expected_spritesheet_portion
        );
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        expected_spritesheet_portion.x = 0.2;
        assert_eq!(
            player_draw_system.spritesheet_portion,
            expected_spritesheet_portion
        );
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        expected_spritesheet_portion.x = 0.4;
        assert_eq!(
            player_draw_system.spritesheet_portion,
            expected_spritesheet_portion
        );
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        expected_spritesheet_portion.x = 0.6;
        assert_eq!(
            player_draw_system.spritesheet_portion,
            expected_spritesheet_portion
        );
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        expected_spritesheet_portion.x = 0.8;
        assert_eq!(
            player_draw_system.spritesheet_portion,
            expected_spritesheet_portion
        );
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        draw(&mut player_draw_system, &config, PhysicsState::MovingRight);
        expected_spritesheet_portion.x = 0.0;
        assert_eq!(
            player_draw_system.spritesheet_portion,
            expected_spritesheet_portion
        );
    }

    fn draw(player_draw_system: &mut PlayerDrawSystem, config: &Config, state: PhysicsState) {
        let (context, _) = &mut initialize(&config).unwrap();
        let images = Images::new(context, &config).unwrap();
        let location = Point2::new(0.0, 0.0);
        player_draw_system
            .draw(&images, &config, context, &location, Some(state))
            .unwrap();
    }
}
