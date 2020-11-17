use std::f32::consts::PI;

use ggez::{
    graphics,
    graphics::DrawMode,
    graphics::{DrawParam, Mesh, MeshBuilder, Rect},
    nalgebra::Point2,
    Context, GameResult,
};
use rand::Rng;

use crate::config::Config;

use super::StaticGameObject;

pub struct TreeTrunks {
    trunk_mesh: Mesh,
    trunk_positions: Vec<Point2<f32>>,
    branch_mesh: Mesh,
}

impl TreeTrunks {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let mut rng = rand::thread_rng();
        let trunk_mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, config.tree_trunk_width, config.tree_trunk_height),
                config.tree_trunk_color,
            )
            .build(context)?;

        let space_between_trees: f32 = (config.resolution_x - config.tree_trunk_count as f32)
            / (config.tree_trunk_count + 1) as f32;
        let mut trunk_positions = vec![];
        for count in 1..=config.tree_trunk_count {
            let offset = rng.gen_range(-config.tree_trunk_shift_by, config.tree_trunk_shift_by);
            let x = count as f32 * space_between_trees + offset;
            let y = config.resolution_y
                - config.bedrock_height
                - config.cave_height
                - config.ground_height
                - config.surface_height
                - config.tree_trunk_height;
            let position = Point2::new(x, y);
            trunk_positions.push(position);
        }

        let branch_mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    0.0,
                    0.0,
                    config.tree_branch_width,
                    config.tree_branch_height,
                ),
                config.tree_trunk_color,
            )
            .build(context)?;

        Ok(TreeTrunks {
            trunk_mesh,
            trunk_positions,
            branch_mesh,
        })
    }
}

impl StaticGameObject for TreeTrunks {
    fn draw(&self, config: &Config, context: &mut Context) -> GameResult {
        self.trunk_positions
            .iter()
            .try_for_each(|tree_trunk_position| {
                graphics::draw(
                    context,
                    &self.trunk_mesh,
                    DrawParam::new().dest([tree_trunk_position.x, tree_trunk_position.y]),
                )?;

                graphics::draw(
                    context,
                    &self.branch_mesh,
                    DrawParam::new()
                        .dest([
                            tree_trunk_position.x + config.tree_branch_width,
                            tree_trunk_position.y + config.tree_branch_width,
                        ])
                        .rotation(config.tree_branch_rotation),
                )?;

                graphics::draw(
                    context,
                    &self.branch_mesh,
                    DrawParam::new()
                        .dest([
                            tree_trunk_position.x + config.tree_trunk_width,
                            tree_trunk_position.y + config.tree_branch_width * 1.5,
                        ])
                        .rotation(-config.tree_branch_rotation),
                )?;

                graphics::draw(
                    context,
                    &self.branch_mesh,
                    DrawParam::new()
                        .dest([
                            tree_trunk_position.x
                                + config.tree_trunk_width / 2.0
                                + config.tree_branch_width / 2.0,
                            tree_trunk_position.y,
                        ])
                        .rotation(PI),
                )
            })
    }
}
