use ggez::{
    graphics::DrawMode,
    graphics::{self, DrawParam, Mesh, MeshBuilder, Rect},
    Context, GameResult,
};

use crate::config::Config;

use super::StaticGameObject;

pub struct Bedrock {
    mesh: Mesh,
}

impl Bedrock {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    0.0,
                    config.resolution_y - config.bedrock_height,
                    config.resolution_x,
                    config.bedrock_height,
                ),
                config.bedrock_color,
            )
            .build(context)?;

        Ok(Bedrock { mesh })
    }
}

impl StaticGameObject for Bedrock {
    fn draw(&self, config: &Config, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.mesh, DrawParam::new())
    }
}
