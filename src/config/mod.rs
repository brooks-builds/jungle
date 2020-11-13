use ggez::event::Button;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(PartialEq, Debug)]
pub struct Config {
    pub resolution_x: f32,
    pub resolution_y: f32,
    pub start_button: Button,
}

#[derive(Serialize, Deserialize)]
struct RawConfig {
    resolution_x: f32,
    resolution_y: f32,
    start_button: String,
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
    };

    Ok(config)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load("test_config.json").unwrap();
        let expected_config = Config {
            resolution_x: 1920.0,
            resolution_y: 1080.0,
            start_button: Button::Start,
        };
        assert_eq!(config, expected_config);
    }
}
