use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Rect, BLACK};
use ggez::nalgebra::Point2;
use ggez::{graphics::Image, Context, GameResult};
use rand::prelude::*;

use crate::config::Config;

pub struct Images {
    pub standing_player: Image,
    pub running_player: Image,
    pub life: Image,
    pub bedrock: Image,
    pub trees: Mesh,
    pub foliage: Mesh,
    pub pit1: Mesh,
    pub ladder: Mesh,
}

impl Images {
    pub fn new(context: &mut Context, config: &Config) -> GameResult<Self> {
        Ok(Images {
            standing_player: Image::new(context, &config.player_standing_image)?,
            running_player: Image::new(context, &config.player_running_spritesheet)?,
            life: Image::new(context, &config.life_image)?,
            bedrock: Image::new(context, &config.bedrock_image)?,
            trees: Self::create_trees(context, config)?,
            foliage: Self::create_foliage(context, config)?,
            pit1: Self::create_single_pit(config, context)?,
            ladder: Self::create_ladder(context, config)?,
        })
    }

    fn create_trees(context: &mut Context, config: &Config) -> GameResult<Mesh> {
        let mut rng = rand::thread_rng();
        let mut mesh = &mut MeshBuilder::new();

        let space_between_trees: f32 = (config.resolution_x - config.tree_trunk_count as f32)
            / (config.tree_trunk_count + 1) as f32;

        for count in 1..config.tree_trunk_count {
            let random_offset =
                rng.gen_range(-config.tree_trunk_shift_by, config.tree_trunk_shift_by);
            let x = count as f32 * space_between_trees + random_offset;
            let y = config.resolution_y
                - config.bedrock_height
                - config.cave_height
                - config.ground_height
                - config.surface_bottom_height
                - config.surface_top_height
                - config.tree_trunk_height;
            mesh = mesh.rectangle(
                DrawMode::fill(),
                Rect::new(x, y, config.tree_trunk_width, config.tree_trunk_height),
                config.tree_trunk_color,
            );

            let points = [
                Point2::new(x, y + config.tree_branch_width),
                Point2::new(x - config.tree_branch_height, y - config.tree_branch_height),
                Point2::new(
                    x - config.tree_branch_height + config.tree_branch_width,
                    y - config.tree_branch_height - config.tree_branch_width,
                ),
                Point2::new(x + config.tree_branch_width, y),
            ];
            mesh = mesh.polyline(DrawMode::fill(), &points, config.tree_trunk_color)?;

            let points = [
                Point2::new(x + config.tree_trunk_width, y + config.tree_branch_width),
                Point2::new(
                    x + config.tree_trunk_width + config.tree_branch_height,
                    y - config.tree_branch_height,
                ),
                Point2::new(
                    x + config.tree_trunk_width + config.tree_branch_height
                        - config.tree_branch_width,
                    y - config.tree_branch_height - config.tree_branch_width,
                ),
                Point2::new(x + config.tree_trunk_width - config.tree_branch_width, y),
            ];
            mesh = mesh.polyline(DrawMode::fill(), &points, config.tree_trunk_color)?;

            mesh = mesh.rectangle(
                DrawMode::fill(),
                Rect::new(
                    x + config.tree_trunk_width / 2.0 - config.tree_branch_width / 2.0,
                    y - config.tree_branch_height,
                    config.tree_branch_width,
                    config.tree_branch_height,
                ),
                config.tree_trunk_color,
            )
        }

        mesh.build(context)
    }

    fn create_foliage(context: &mut Context, config: &Config) -> GameResult<Mesh> {
        let y = config.resolution_y
            - config.bedrock_height
            - config.cave_height
            - config.ground_height
            - config.surface_bottom_height
            - config.surface_top_height
            - config.tree_trunk_height;
        let mut sin_step: f32 = 0.0;
        let section_width = config.resolution_x / config.foliage_points as f32;
        let mut mesh = &mut MeshBuilder::new();

        for count in 0..config.foliage_points {
            mesh = mesh.rectangle(
                DrawMode::fill(),
                Rect::new(
                    count as f32 * section_width,
                    0.0,
                    section_width,
                    y + sin_step.sin() * config.foliage_step_vertical,
                ),
                config.foliage_color,
            );
            sin_step += 1.0;
        }

        mesh.build(context)
    }

    fn create_single_pit(config: &Config, context: &mut Context) -> GameResult<Mesh> {
        MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    -(config.pit_width / 2.0),
                    -(config.pit_height / 2.0),
                    config.pit_width,
                    config.pit_height,
                ),
                config.pit_color,
            )
            .build(context)
    }

    pub fn reset_trees(&mut self, context: &mut Context, config: &Config) -> GameResult {
        self.trees = Self::create_trees(context, config)?;

        Ok(())
    }

    fn create_ladder(context: &mut Context, config: &Config) -> GameResult<Mesh> {
        let height = config.cave_height + config.ground_height + config.surface_bottom_height;

        let rung_count = height / (config.ladder_rung_spacing + config.ladder_rung_height);

        let mut mesh = &mut MeshBuilder::new();
        let rung = Rect::new(
            config.resolution_x / 2.0 - config.ladder_width / 2.0,
            config.resolution_y - height - config.bedrock_height,
            config.ladder_width,
            config.ladder_rung_height,
        );
        let combined_rung_height = config.ladder_rung_spacing + config.ladder_rung_height;

        mesh = mesh.rectangle(
            DrawMode::fill(),
            Rect::new(
                rung.x - config.ladder_margin,
                rung.y - config.ladder_margin,
                config.ladder_width + config.ladder_margin * 2.0,
                height,
            ),
            BLACK,
        );
        for count in 0..rung_count as u32 {
            let mut new_rung = rung;
            new_rung.y += combined_rung_height * count as f32;
            mesh = mesh.rectangle(DrawMode::fill(), new_rung, config.ladder_rung_color);
        }

        mesh.build(context)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{config, initialize::initialize};

    #[test]
    fn test_create_images() {
        let config = config::load("config.json").unwrap();
        let (context, _event_loop) = &mut initialize(&config).unwrap();
        let images = Images::new(context, &config).unwrap();
        let standing_player_image =
            ggez::graphics::Image::new(context, &config.player_standing_image).unwrap();
        let running_player_spritesheet =
            Image::new(context, &config.player_running_spritesheet).unwrap();
        let life_image = Image::new(context, &config.life_image).unwrap();
        assert_eq!(
            standing_player_image.to_rgba8(context).unwrap(),
            images.standing_player.to_rgba8(context).unwrap()
        );
        assert_eq!(
            running_player_spritesheet.to_rgba8(context).unwrap(),
            images.running_player.to_rgba8(context).unwrap()
        );
        assert_eq!(
            life_image.to_rgba8(context).unwrap(),
            images.life.to_rgba8(context).unwrap()
        );
    }
}
