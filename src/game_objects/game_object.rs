use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

use crate::config::Config;
use crate::draw_systems::DrawSystem;
use crate::handle_input::Command;
use crate::images::Images;
use crate::life_systems::LifeSystem;
use crate::physics_systems::PhysicsSystem;

use super::game_object_types::GameObjectfeatureTypes;
use super::GameObjectTypes;

pub struct GameObject {
    pub location: Point2<f32>,
    pub width: f32,
    pub height: f32,
    draw_system: Option<Box<dyn DrawSystem>>,
    life_system: Option<Box<dyn LifeSystem>>,
    physics_system: Option<Box<dyn PhysicsSystem>>,
    pub my_type: GameObjectTypes,
    pub feature_type: Option<GameObjectfeatureTypes>,
}

impl GameObject {
    pub fn draw(
        &mut self,
        context: &mut Context,
        config: &Config,
        images: &mut Images,
    ) -> GameResult {
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

    pub fn update(&mut self, command: Option<Command>, features: Vec<GameObject>) {
        if let Some(physics_system) = &mut self.physics_system {
            physics_system.update(&mut self.location, command, features);
        }
    }

    pub fn is_offscreen_right(&self, screen_width: f32) -> bool {
        self.location.x - self.width / 2.0 >= screen_width
    }

    pub fn is_offscreen_left(&self) -> bool {
        self.location.x + self.width / 2.0 <= 0.0
    }
}

impl Clone for GameObject {
    fn clone(&self) -> Self {
        Self {
            location: self.location,
            width: self.width,
            height: self.height,
            draw_system: None,
            life_system: None,
            physics_system: None,
            my_type: self.my_type,
            feature_type: self.feature_type,
        }
    }
}

#[derive(Debug)]
pub enum GameObjectBuilderError {
    MyTypeNotSet,
}

impl std::fmt::Display for GameObjectBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameObjectBuilderError::MyTypeNotSet => {
                write!(f, "Type not set when building new game object")
            }
        }
    }
}

pub struct GameObjectBuilder {
    location: Point2<f32>,
    width: f32,
    draw_system: Option<Box<dyn DrawSystem>>,
    life_system: Option<Box<dyn LifeSystem>>,
    physics_system: Option<Box<dyn PhysicsSystem>>,
    my_type: Option<GameObjectTypes>,
    height: f32,
    feature_type: Option<GameObjectfeatureTypes>,
}

impl GameObjectBuilder {
    pub fn new() -> Self {
        Self {
            location: Point2::new(0.0, 0.0),
            width: 0.0,
            draw_system: None,
            life_system: None,
            physics_system: None,
            my_type: None,
            height: 0.0,
            feature_type: None,
        }
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

    pub fn with_type(mut self, game_object_type: GameObjectTypes) -> Self {
        self.my_type = Some(game_object_type);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn with_feature_type(mut self, feature_type: GameObjectfeatureTypes) -> Self {
        self.feature_type = Some(feature_type);
        self
    }

    pub fn build(self) -> Result<GameObject, GameObjectBuilderError> {
        let my_type = if let Some(game_object_type) = self.my_type {
            game_object_type
        } else {
            return Err(GameObjectBuilderError::MyTypeNotSet);
        };

        Ok(GameObject {
            location: self.location,
            width: self.width,
            height: self.height,
            draw_system: self.draw_system,
            life_system: self.life_system,
            physics_system: self.physics_system,
            my_type,
            feature_type: self.feature_type,
        })
    }
}

#[cfg(test)]
mod test {
    use ggez::nalgebra::Point2;

    use crate::draw_systems::player_draw_system::PlayerDrawSystem;
    use crate::game_objects::game_object_types::GameObjectfeatureTypes;
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
        let height = 50.0;
        let player: GameObject = GameObjectBuilder::new()
            .location(Point2::new(x, y))
            .width(width)
            .draw_system(Box::new(PlayerDrawSystem::new(&config)))
            .life_system(Box::new(PlayerLifeSystem::new(lives)))
            .with_type(GameObjectTypes::Player)
            .physics_system(Box::new(PlayerPhysicsSystem::new(&config)))
            .height(height)
            .build()
            .unwrap();

        assert_eq!(player.location.x, x);
        assert_eq!(player.location.y, y);
        assert_eq!(player.width, width);
        assert_eq!(player.my_type, GameObjectTypes::Player);
        assert_eq!(player.height, height);
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
            .with_type(GameObjectTypes::Player)
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
            .with_type(GameObjectTypes::Player)
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
            .with_type(GameObjectTypes::Player)
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
            .with_type(GameObjectTypes::Player)
            .build()
            .unwrap();
        assert_eq!(game_object.is_offscreen_left(), false);
    }

    #[test]
    fn ci_test_creating_pit1() {
        let pit1 = GameObjectBuilder::new()
            .with_type(GameObjectTypes::Feature)
            .with_feature_type(GameObjectfeatureTypes::Pit1)
            .build()
            .unwrap();

        assert_eq!(pit1.feature_type, Some(GameObjectfeatureTypes::Pit1));
    }
}
