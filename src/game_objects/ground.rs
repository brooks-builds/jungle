use ggez::{
    graphics::DrawMode,
    graphics::{self, DrawParam, Mesh, MeshBuilder, Rect},
    Context, GameResult,
};

use crate::config::Config;

use super::StaticGameObject;

pub struct Ground {
    mesh: Mesh,
}

impl Ground {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    0.0,
                    config.resolution_y
                        - config.bedrock_height
                        - config.cave_height
                        - config.ground_height,
                    config.resolution_x,
                    config.ground_height,
                ),
                config.ground_color,
            )
            .build(context)?;

        Ok(Ground { mesh })
    }
}

impl StaticGameObject for Ground {
    fn draw(&self, config: &Config, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.mesh, DrawParam::new())
    }
}
