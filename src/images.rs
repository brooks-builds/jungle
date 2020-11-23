use ggez::{graphics::Image, Context, GameResult};

use crate::config::Config;

pub struct Images {
    pub standing_player: Image,
    pub running_player: Image,
}

impl Images {
    pub fn new(context: &mut Context, config: &Config) -> GameResult<Self> {
        let standing_player = Image::new(context, &config.player_standing_image)?;
        let running_player = Image::new(context, &config.player_running_spritesheet)?;

        Ok(Images {
            standing_player,
            running_player,
        })
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
        assert_eq!(
            standing_player_image.to_rgba8(context).unwrap(),
            images.standing_player.to_rgba8(context).unwrap()
        );
        assert_eq!(
            running_player_spritesheet.to_rgba8(context).unwrap(),
            images.running_player.to_rgba8(context).unwrap()
        );
    }
}
