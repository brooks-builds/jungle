use std::f32::consts::PI;

use ggez::{
    event::Button,
    graphics::DrawParam,
    graphics::{self, DrawMode, Font, Mesh, MeshBuilder, Rect, Scale, Text},
    nalgebra::Point2,
    Context, GameResult,
};
use rand::Rng;

use crate::config::Config;

use super::Scene;

#[derive(Debug)]
pub struct MainScene {
    map_index: usize,
    bedrock: Mesh,
    ground: Mesh,
    surface: Mesh,
    tree_trunk: Mesh,
    tree_trunk_positions: Vec<Point2<f32>>,
    tree_branch: Mesh,
    background: Mesh,
    leaves: Mesh,
}

impl MainScene {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let mut rng = rand::thread_rng();
        let map_index = config.start_index;
        let bedrock = MeshBuilder::new()
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
        let ground = MeshBuilder::new()
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
        let surface = MeshBuilder::new()
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
        let mut tree_trunk_positions = vec![];
        let tree_trunk = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, config.tree_trunk_width, config.tree_trunk_height),
                config.tree_trunk_color,
            )
            .build(context)?;

        let space_between_trees: f32 = (config.resolution_x - config.tree_trunk_count as f32)
            / (config.tree_trunk_count + 1) as f32;
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
            tree_trunk_positions.push(position);
        }

        let tree_branch = MeshBuilder::new()
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

        let background = MeshBuilder::new()
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

        let tree_line_y = config.resolution_y
            - config.bedrock_height
            - config.cave_height
            - config.ground_height
            - config.surface_height
            - config.tree_trunk_height;
        let mut leaf_points = vec![
            Point2::new(0.0, 0.0),
            Point2::new(config.resolution_x, 0.0),
            Point2::new(
                config.resolution_x,
                tree_line_y + rng.gen_range(-config.leaves_shift_by, config.leaves_shift_by),
            ),
        ];

        // put leaf points in the middle of the screen

        leaf_points.push(Point2::new(
            0.0,
            tree_line_y + rng.gen_range(-config.leaves_shift_by, config.leaves_shift_by),
        ));
        let leaf_space_between = config.resolution_x / config.leaf_points as f32;

        let leaves = MeshBuilder::new()
            .polygon(DrawMode::fill(), &leaf_points, config.leaf_color)?
            .build(context)?;

        Ok(MainScene {
            map_index,
            bedrock,
            ground,
            surface,
            tree_trunk,
            tree_trunk_positions,
            tree_branch,
            background,
            leaves,
        })
    }
}

impl Scene for MainScene {
    fn update(
        &mut self,
        _context: &mut Context,
        _button_pressed: Option<Button>,
        _config: &Config,
        _active_scene: &mut super::ActiveScene,
    ) -> GameResult {
        Ok(())
    }

    fn draw(&self, context: &mut Context, config: &Config) -> GameResult {
        graphics::draw(context, &self.background, DrawParam::new())?;
        graphics::draw(context, &self.bedrock, DrawParam::new())?;
        graphics::draw(context, &self.ground, DrawParam::new())?;
        graphics::draw(context, &self.surface, DrawParam::new())?;

        self.tree_trunk_positions
            .iter()
            .try_for_each(|tree_trunk_position| {
                graphics::draw(
                    context,
                    &self.tree_trunk,
                    DrawParam::new().dest([tree_trunk_position.x, tree_trunk_position.y]),
                )?;

                graphics::draw(
                    context,
                    &self.tree_branch,
                    DrawParam::new()
                        .dest([
                            tree_trunk_position.x + config.tree_branch_width,
                            tree_trunk_position.y + config.tree_branch_width,
                        ])
                        .rotation(config.tree_branch_rotation),
                )?;

                graphics::draw(
                    context,
                    &self.tree_branch,
                    DrawParam::new()
                        .dest([
                            tree_trunk_position.x + config.tree_trunk_width,
                            tree_trunk_position.y + config.tree_branch_width * 1.5,
                        ])
                        .rotation(-config.tree_branch_rotation),
                )?;

                graphics::draw(
                    context,
                    &self.tree_branch,
                    DrawParam::new()
                        .dest([
                            tree_trunk_position.x
                                + config.tree_trunk_width / 2.0
                                + config.tree_branch_width / 2.0,
                            tree_trunk_position.y,
                        ])
                        .rotation(PI),
                )
            })?;

        graphics::draw(context, &self.leaves, DrawParam::new())?;

        Ok(())
    }
}
