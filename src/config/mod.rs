use ggez::event::Button;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(PartialEq, Debug)]
pub struct Config {
    pub resolution_x: f32,
    pub resolution_y: f32,
    pub start_button: Button,
    pub title: String,
    pub title_subtext: String,
    pub font_large: f32,
    pub font_medium: f32,
    pub font_small: f32,
}

#[derive(Serialize, Deserialize)]
struct RawConfig {
    resolution_x: f32,
    resolution_y: f32,
    start_button: String,
    title: String,
    title_subtext: String,
    font_large: f32,
    font_medium: f32,
    font_small: f32,
}

pub fn load(file_name: &str) -> eyre::Result<Config> {
    let mut config_file = File::open(file_name)?;
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string)?;

    let raw_config: RawConfig = serde_json::from_str(&config_string)?;

    let start_button = match raw_config.start_button.as_str() {
        "Start" => Button::Start,
        _ => Button::Start,
    };

    let config = Config {
        resolution_x: raw_config.resolution_x,
        resolution_y: raw_config.resolution_y,
        start_button,
        title: raw_config.title,
        title_subtext: raw_config.title_subtext,
        font_large: raw_config.font_large,
        font_medium: raw_config.font_medium,
        font_small: raw_config.font_small,
    };

    Ok(config)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load("config.json").unwrap();
        let expected_config = Config {
            resolution_x: 1920.0,
            resolution_y: 1080.0,
            start_button: Button::Start,
            title: "Jungle".to_owned(),
            title_subtext: "Press start to begin".to_owned(),
            font_large: 72.0,
            font_medium: 55.0,
            font_small: 36.0,
        };
        assert_eq!(config, expected_config);
    }
}
