use ggez::graphics;
use graphics::spritebatch::{SpriteBatch, SpriteIdx};
use graphics::{DrawParam, Image};

use super::DrawSystem;

pub struct HeartDrawSystem {
    hearts: SpriteBatch,
    lives: u8,
    x: f32,
    y: f32,
    width: f32,
    spritebatch_indexes: Vec<SpriteIdx>,
}

impl HeartDrawSystem {
    pub fn new(heart_image: Image) -> Self {
        Self {
            hearts: SpriteBatch::new(heart_image),
            lives: 3,
            x: 0.0,
            y: 0.0,
            width: 50.0,
            spritebatch_indexes: vec![],
        }
    }

    pub fn build(mut self) -> Self {
        for count in 0..self.lives {
            let index = self
                .hearts
                .add(DrawParam::new().dest([self.x + count as f32 * self.width, self.y]));
            self.spritebatch_indexes.push(index);
        }
        self
    }

    pub fn set_lives(mut self, lives: u8) -> Self {
        self.lives = lives;
        self
    }

    pub fn set_location(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl DrawSystem for HeartDrawSystem {
    fn draw(
        &mut self,
        _images: &mut crate::images::Images,
        _config: &crate::config::Config,
        context: &mut ggez::Context,
        _location: &ggez::nalgebra::Point2<f32>,
        _physics_system: Option<crate::physics_systems::PhysicsState>,
        _life_system: &Option<Box<dyn crate::life_systems::LifeSystem>>,
    ) -> ggez::GameResult {
        graphics::draw(context, &self.hearts, DrawParam::new())
    }
}

#[cfg(test)]
mod tests {
    use ggez::graphics::Image;

    use crate::config;
    use crate::initialize::initialize;

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_create_heart_draw_system() {
        let config = &config::load("config.json").unwrap();
        let (context, _) = &mut initialize(config).unwrap();
        let heart_image = Image::new(context, &config.life_image).unwrap();
        let heart_draw_system = HeartDrawSystem::new(heart_image);

        assert_eq!(heart_draw_system.lives, 3);
        assert_eq!(heart_draw_system.x, 0.0);
        assert_eq!(heart_draw_system.y, 0.0);
        assert_eq!(heart_draw_system.width, 50.0);
    }

    #[test]
    fn test_building_heart_draw_system() {
        let config = &config::load("config.json").unwrap();
        let (context, _) = &mut initialize(config).unwrap();
        let heart_image = Image::new(context, &config.life_image).unwrap();
        let heart_draw_system = HeartDrawSystem::new(heart_image).build();
        assert_eq!(heart_draw_system.spritebatch_indexes.len(), 3);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_create_heart_draw_system_with_overrides() {
        let config = &config::load("config.json").unwrap();
        let (context, _) = &mut initialize(config).unwrap();
        let heart_image = Image::new(context, &config.life_image).unwrap();
        let heart_draw_system = HeartDrawSystem::new(heart_image)
            .set_lives(5)
            .set_location(50.0, 55.0)
            .set_width(100.0);

        assert_eq!(heart_draw_system.lives, 5);
        assert_eq!(heart_draw_system.x, 50.0);
        assert_eq!(heart_draw_system.y, 55.0);
        assert_eq!(heart_draw_system.width, 100.0);
    }
}
