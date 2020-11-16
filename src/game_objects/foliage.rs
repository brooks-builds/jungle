use ggez::{
    graphics::DrawMode,
    graphics::{self, DrawParam, Mesh, MeshBuilder},
    nalgebra::Point2,
    Context, GameResult,
};

use crate::config::Config;

use super::StaticGameObject;

pub struct Foliage {
    mesh: Mesh,
}

impl Foliage {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let mut points = vec![
            Point2::new(0.0, 0.0),
            Point2::new(config.resolution_x, 0.0),
            Point2::new(
                config.resolution_x,
                config.resolution_y
                    - config.bedrock_height
                    - config.cave_height
                    - config.ground_height
                    - config.surface_height
                    - config.tree_trunk_height
                    // put this 3 into the config
                    + 3.0 * config.foliage_step,
            ),
            Point2::new(
                0.0,
                config.resolution_y
                    - config.bedrock_height
                    - config.cave_height
                    - config.ground_height
                    - config.surface_height
                    - config.tree_trunk_height,
            ),
        ];
        let mesh = MeshBuilder::new()
            .polygon(DrawMode::fill(), &points, config.foliage_color)?
            .build(context)?;

        Ok(Foliage { mesh })
    }
}

impl StaticGameObject for Foliage {
    fn draw(&self, config: &Config, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.mesh, DrawParam::new())
    }
}
