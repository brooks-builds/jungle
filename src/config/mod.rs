mod map;
mod serde_button;
mod serde_color;

use ggez::graphics::BLACK;
use ggez::{event::Button, graphics::Color};
use serde::{Deserialize, Serialize};
use std::fs::File;

pub use self::map::MapFeature;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "crate::config::serde_color")]
    pub bedrock_color: Color,
    pub bedrock_height: f32,
    pub bedrock_image: String,
    pub cave_height: f32,
    #[serde(with = "crate::config::serde_color")]
    pub foliage_color: Color,
    pub foliage_points: u8,
    pub foliage_step_vertical: f32,
    pub font_large: f32,
    pub font_medium: f32,
    pub font_small: f32,
    #[serde(with = "crate::config::serde_color")]
    pub ground_color: Color,
    pub ground_height: f32,
    #[serde(with = "crate::config::serde_button")]
    pub jump_button: Button,
    pub jump_force: f32,
    pub life_image: String,
    pub life_width: f32,
    pub map: Vec<Vec<MapFeature>>,
    #[serde(with = "crate::config::serde_button")]
    pub move_left_button: Button,
    #[serde(with = "crate::config::serde_button")]
    pub move_right_button: Button,
    #[serde(with = "crate::config::serde_color")]
    pub pit_color: Color,
    pub pit_height: f32,
    pub pit_width: f32,
    pub player_height: f32,
    pub player_lives: u8,
    pub player_running_spritesheet_count: f32,
    pub player_running_spritesheet: String,
    pub player_speed: f32,
    pub player_standing_image_height: f32,
    pub player_standing_image_width: f32,
    pub player_standing_image: String,
    pub player_starting_x: f32,
    pub player_starting_y: f32,
    pub player_width: f32,
    pub resolution_x: f32,
    pub resolution_y: f32,
    #[serde(with = "crate::config::serde_color")]
    pub sky_color: Color,
    pub spritesheet_animation_speed: u8,
    #[serde(with = "crate::config::serde_button")]
    pub start_button: Button,
    pub start_index: usize,
    pub surface_bottom_height: f32,
    #[serde(with = "crate::config::serde_color")]
    pub surface_color: Color,
    pub surface_top_height: f32,
    pub title_subtext: String,
    pub title: String,
    pub tree_branch_height: f32,
    pub tree_branch_rotation: f32,
    pub tree_branch_width: f32,
    #[serde(with = "crate::config::serde_color")]
    pub tree_trunk_color: Color,
    pub tree_trunk_count: u8,
    pub tree_trunk_height: f32,
    pub tree_trunk_shift_by: f32,
    pub tree_trunk_start: f32,
    pub tree_trunk_width: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bedrock_color: Color::from_rgb(147, 94, 59),
            bedrock_height: 15.0,
            bedrock_image: "/bedrock.png".to_owned(),
            cave_height: 200.0,
            foliage_color: Color::from_rgb(33, 43, 18),
            foliage_points: 50,
            foliage_step_vertical: 15.0,
            font_large: 72.0,
            font_medium: 55.0,
            font_small: 36.0,
            ground_color: Color::from_rgb(146, 137, 60),
            ground_height: 50.0,
            jump_button: Button::South,
            jump_force: 25.0,
            life_image: "/heart.png".to_owned(),
            life_width: 32.0,
            map: vec![vec![MapFeature::Pit1]],
            move_left_button: Button::DPadLeft,
            move_right_button: Button::DPadRight,
            pit_color: BLACK,
            pit_height: 100.0,
            pit_width: 125.0,
            player_height: 160.0,
            player_lives: 7,
            player_running_spritesheet_count: 5.0,
            player_running_spritesheet: "jungle_player_running-Sheet.png".to_owned(),
            player_speed: 10.0,
            player_standing_image_height: 160.0,
            player_standing_image_width: 160.0,
            player_standing_image: "/jungle_player_standing.png".to_owned(),
            player_starting_x: 150.0,
            player_starting_y: 675.0,
            player_width: 80.0,
            resolution_x: 1920.0,
            resolution_y: 1080.0,
            sky_color: Color::from_rgb(164, 196, 112),
            spritesheet_animation_speed: 10,
            start_button: Button::Start,
            start_index: 0,
            surface_bottom_height: 25.0,
            surface_color: Color::from_rgb(189, 179, 94),
            surface_top_height: 125.0,
            title_subtext: "Press start to begin".to_owned(),
            title: "Jungle".to_owned(),
            tree_branch_height: 50.0,
            tree_branch_rotation: 2.35,
            tree_branch_width: 10.0,
            tree_trunk_color: Color::from_rgb(74, 69, 31),
            tree_trunk_count: 9,
            tree_trunk_height: 300.0,
            tree_trunk_shift_by: 25.0,
            tree_trunk_start: 150.0,
            tree_trunk_width: 50.0,
        }
    }
}

pub fn load(file_name: &str) -> eyre::Result<Config> {
    let config = serde_json::from_reader(File::open(file_name)?)?;

    Ok(config)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ci_test_load_config() {
        load("config.json").unwrap();
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_default_config() {
        let config: Config = Config::default();

        assert_eq!(config.tree_trunk_height, 300.0)
    }
}
