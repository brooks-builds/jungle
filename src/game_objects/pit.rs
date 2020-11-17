use ggez::{
    graphics::DrawMode,
    graphics::{self, DrawParam},
    graphics::{Mesh, MeshBuilder, Rect},
    nalgebra::Point2,
    Context, GameResult,
};

use crate::config::Config;

use super::StaticGameObject;

pub struct Pit {
    mesh: Mesh,
    location: Point2<f32>,
}

impl Pit {
    pub fn new(config: &Config, context: &mut Context, location: Point2<f32>) -> GameResult<Self> {
        let mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    0.0,
                    0.0,
                    config.pit_width,
                    config.surface_height - config.pit_margin * 2.0,
                ),
                config.pit_color,
            )
            .build(context)?;

        Ok(Pit { mesh, location })
    }
}

impl StaticGameObject for Pit {
    fn draw(&self, _config: &Config, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.mesh, DrawParam::new().dest(self.location))
    }
}
