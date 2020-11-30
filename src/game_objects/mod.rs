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
    life_systems::LifeSystem, physics_systems::PhysicsSystem,
};

#[derive(PartialEq, Debug, Eq, Hash)]
pub enum GameObjectTypes {
    Player,
    Heart,
}

pub trait StaticGameObject {
    fn draw(&self, config: &Config, context: &mut Context) -> GameResult;
}

pub struct GameObject {
    pub location: Point2<f32>,
    pub width: f32,
    draw_system: Option<Box<dyn DrawSystem>>,
    life_system: Option<Box<dyn LifeSystem>>,
    physics_system: Option<Box<dyn PhysicsSystem>>,
}

impl GameObject {
    pub fn draw(&mut self, context: &mut Context, config: &Config, images: &Images) -> GameResult {
        let physics_state = if let Some(physics_system) = &self.physics_system {
            Some(physics_system.get_state())
        } else {
            None
        };

        if let Some(draw_system) = &mut self.draw_system {
            draw_system.draw(
                images,
                config,
                context,
                &self.location,
                physics_state,
                &self.life_system,
            )?;
        }

        Ok(())
    }

    pub fn update(&mut self, command: Option<Command>) {
        if let Some(physics_system) = &mut self.physics_system {
            physics_system.update(&mut self.location, command);
        }
    }

    pub fn is_offscreen_right(&self, screen_width: f32) -> bool {
        self.location.x - self.width / 2.0 >= screen_width
    }

    pub fn is_offscreen_left(&self) -> bool {
        self.location.x + self.width / 2.0 <= 0.0
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GameObjectBuilderError {}

pub struct GameObjectBuilder {
    location: Point2<f32>,
    width: f32,
    draw_system: Option<Box<dyn DrawSystem>>,
    life_system: Option<Box<dyn LifeSystem>>,
    physics_system: Option<Box<dyn PhysicsSystem>>,
}

impl GameObjectBuilder {
    pub fn new() -> Self {
        Self {
            location: Point2::new(0.0, 0.0),
            width: 0.0,
            draw_system: None,
            life_system: None,
            physics_system: None,
        }
    }

    pub fn as_type(mut self, new_type: GameObjectTypes) -> Self {
        self
    }

    pub fn location(mut self, location: Point2<f32>) -> Self {
        self.location = location;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn draw_system(mut self, draw_system: Box<dyn DrawSystem>) -> Self {
        self.draw_system = Some(draw_system);
        self
    }

    pub fn life_system(mut self, life_system: Box<dyn LifeSystem>) -> Self {
        self.life_system = Some(life_system);
        self
    }

    pub fn physics_system(mut self, physics_system: Box<dyn PhysicsSystem>) -> Self {
        self.physics_system = Some(physics_system);
        self
    }

    pub fn build(self) -> Result<GameObject, GameObjectBuilderError> {
        Ok(GameObject {
            location: self.location,
            width: self.width,
            draw_system: self.draw_system,
            life_system: self.life_system,
            physics_system: self.physics_system,
        })
    }
}

#[cfg(test)]
mod test {
    use ggez::nalgebra::Point2;

    use crate::draw_systems::hearts_draw_system::HeartDrawSystem;
    use crate::draw_systems::player_draw_system::PlayerDrawSystem;
    use crate::{
        config, life_systems::player_life_system::PlayerLifeSystem,
        physics_systems::player_physics_system::PlayerPhysicsSystem,
    };

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_use_builder_to_create_player_game_object() {
        let x = 50.0;
        let y = 55.0;
        let width = 100.0;
        let lives = 3;
        let config = config::load("config.json").unwrap();
        let player: GameObject = GameObjectBuilder::new()
            .location(Point2::new(x, y))
            .width(width)
            .draw_system(Box::new(PlayerDrawSystem::new(&config)))
            .life_system(Box::new(PlayerLifeSystem::new(lives)))
            .physics_system(Box::new(PlayerPhysicsSystem::new(&config)))
            .build()
            .unwrap();

        assert_eq!(player.location.x, x);
        assert_eq!(player.location.y, y);
        assert_eq!(player.width, width);
        player.life_system.unwrap();
        player.physics_system.unwrap();
    }

    #[test]
    fn ci_test_is_offscreen_right() {
        let location = Point2::new(52.5, 50.0);
        let width = 5.0;
        let game_object = GameObjectBuilder::new()
            .location(location)
            .width(width)
            .build()
            .unwrap();
        let screen_width = 50.0;
        assert_eq!(game_object.is_offscreen_right(screen_width), true);
    }

    #[test]
    fn ci_test_is_not_offscreen_right() {
        let location = Point2::new(25.0, 50.0);
        let width = 5.0;
        let game_object = GameObjectBuilder::new()
            .location(location)
            .width(width)
            .build()
            .unwrap();
        let screen_width = 50.0;
        assert_eq!(game_object.is_offscreen_right(screen_width), false);
    }

    #[test]
    fn ci_test_is_offscreen_left() {
        let location = Point2::new(-2.5, 50.0);
        let width = 5.0;
        let game_object = GameObjectBuilder::new()
            .location(location)
            .width(width)
            .build()
            .unwrap();
        assert_eq!(game_object.is_offscreen_left(), true);
    }

    #[test]
    fn ci_test_is_not_offscreen_left() {
        let location = Point2::new(0.0, 50.0);
        let width = 5.0;
        let game_object = GameObjectBuilder::new()
            .location(location)
            .width(width)
            .build()
            .unwrap();
        assert_eq!(game_object.is_offscreen_left(), false);
    }
}
