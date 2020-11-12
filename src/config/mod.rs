use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Config {
    pub resolution_x: f32,
    pub resolution_y: f32,
}

pub fn load(file_name: &str) -> eyre::Result<Config> {
    let mut config_file = File::open(file_name)?;
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string)?;

    let config: Config = serde_json::from_str(&config_string)?;

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
        };
        assert_eq!(config, expected_config);
    }
}
