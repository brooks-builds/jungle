use ggez::conf::Backend;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Color, DrawParam, Image};
use ggez::nalgebra::Point2;

use super::DrawSystem;

pub struct BackgroundDrawSystem {
    grounds_spritebatch: SpriteBatch,
}

impl BackgroundDrawSystem {
    pub fn new(bedrock_image: Image) -> Self {
        let grounds_spritebatch = SpriteBatch::new(bedrock_image);

        Self {
            grounds_spritebatch,
        }
    }

    pub fn bedrock(mut self, location: Point2<f32>, color: Color) -> Self {
        self.grounds_spritebatch
            .add(DrawParam::new().dest([location.x, location.y]).color(color));
        self
    }

    pub fn ground(mut self, location: Point2<f32>, color: Color, scale_y: f32) -> Self {
        self.grounds_spritebatch.add(
            DrawParam::new()
                .dest([location.x, location.y])
                .color(color)
                .scale([1.0, scale_y]),
        );
        self
    }
}

impl DrawSystem for BackgroundDrawSystem {
    fn draw(
        &mut self,
        images: &mut crate::images::Images,
        config: &crate::config::Config,
        context: &mut ggez::Context,
        location: &ggez::nalgebra::Point2<f32>,
        physics_system: Option<crate::physics_systems::PhysicsState>,
        life_system: &Option<Box<dyn crate::life_systems::LifeSystem>>,
    ) -> ggez::GameResult {
        ggez::graphics::draw(context, &self.grounds_spritebatch, DrawParam::new())
    }
}

#[cfg(test)]
mod tests {
    use ggez::graphics::WHITE;

    use crate::images::Images;
    use crate::{config, initialize};

    use super::*;

    #[test]
    fn test_create_new_background_draw_system() {
        let config = config::load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let images = Images::new(context, &config).unwrap();
        let background_draw_system = BackgroundDrawSystem::new(images.bedrock.clone());
    }

    #[test]
    fn test_setting_bedrock_in_background_draw_system() {
        let config = config::load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let images = Images::new(context, &config).unwrap();
        let background_draw_system =
            BackgroundDrawSystem::new(images.bedrock.clone()).bedrock(Point2::new(0.3, 0.5), WHITE);
    }

    #[test]
    fn test_setting_ground_in_background_draw_system() {
        let config = config::load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let images = Images::new(context, &config).unwrap();
        let background_draw_system = BackgroundDrawSystem::new(images.bedrock.clone()).ground(
            Point2::new(0.3, 0.5),
            WHITE,
            1.5,
        );
    }
}
