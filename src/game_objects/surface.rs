use ggez::{
    graphics::DrawMode,
    graphics::{self, DrawParam, Mesh, MeshBuilder, Rect},
    Context, GameResult,
};

use crate::config::Config;

use super::StaticGameObject;

pub struct Surface {
    mesh: Mesh,
}

impl Surface {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    0.0,
                    config.resolution_y
                        - config.bedrock_height
                        - config.cave_height
                        - config.ground_height
                        - config.surface_height,
                    config.resolution_x,
                    config.surface_height,
                ),
                config.surface_color,
            )
            .build(context)?;

        Ok(Surface { mesh })
    }
}

impl StaticGameObject for Surface {
    fn draw(&self, config: &Config, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.mesh, DrawParam::new())
    }
}
