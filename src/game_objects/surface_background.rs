use ggez::{
    graphics::DrawMode,
    graphics::{self, DrawParam, Mesh, MeshBuilder, Rect},
    Context, GameResult,
};

use crate::config::Config;

use super::StaticGameObject;

pub struct SurfaceBackground {
    mesh: Mesh,
}

impl SurfaceBackground {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    0.0,
                    0.0,
                    config.resolution_x,
                    config.resolution_y
                        - config.bedrock_height
                        - config.cave_height
                        - config.ground_height
                        - config.surface_height,
                ),
                config.background_color,
            )
            .build(context)?;

        Ok(SurfaceBackground { mesh })
    }
}

impl StaticGameObject for SurfaceBackground {
    fn draw(&self, _config: &Config, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.mesh, DrawParam::new())
    }
}
