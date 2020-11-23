mod map;
mod serde_button;
mod serde_color;

use ggez::{event::Button, graphics::Color};
use serde::{Deserialize, Serialize};
use std::fs::File;

use self::map::ConfigMap;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Config {
    pub player_running_spritesheet_count: f32,
    pub resolution_x: f32,
    pub resolution_y: f32,
    pub spritesheet_animation_speed: u8,
    #[serde(with = "crate::config::serde_button")]
    pub start_button: Button,
    #[serde(with = "crate::config::serde_button")]
    pub move_right_button: Button,
    #[serde(with = "crate::config::serde_button")]
    pub move_left_button: Button,
    pub title: String,
    pub title_subtext: String,
    pub font_large: f32,
    pub font_medium: f32,
    pub font_small: f32,
    pub map: Vec<ConfigMap>,
    pub start_index: usize,
    pub bedrock_height: f32,
    #[serde(with = "crate::config::serde_color")]
    pub bedrock_color: Color,
    pub cave_height: f32,
    pub ground_height: f32,
    #[serde(with = "crate::config::serde_color")]
    pub ground_color: Color,
    pub surface_height: f32,
    #[serde(with = "crate::config::serde_color")]
    pub surface_color: Color,
    #[serde(with = "crate::config::serde_color")]
    pub tree_trunk_color: Color,
    pub tree_trunk_height: f32,
    pub tree_trunk_width: f32,
    pub tree_trunk_count: u8,
    pub tree_trunk_start: f32,
    pub tree_trunk_shift_by: f32,
    pub tree_branch_width: f32,
    pub tree_branch_height: f32,
    pub tree_branch_rotation: f32,
    #[serde(with = "crate::config::serde_color")]
    pub background_color: Color,
    pub foliage_points: u8,
    pub foliage_step_vertical: f32,
    #[serde(with = "crate::config::serde_color")]
    pub foliage_color: Color,
    pub pit_width: f32,
    pub pit_margin: f32,
    #[serde(with = "crate::config::serde_color")]
    pub pit_color: Color,
    pub player_standing_image: String,
    pub player_running_spritesheet: String,
    pub player_starting_x: f32,
    pub player_starting_y: f32,
    pub player_speed: f32,
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
}
