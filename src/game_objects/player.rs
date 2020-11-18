use ggez::{
    graphics::DrawParam,
    graphics::{self, Image, Scale},
    Context, GameResult,
};

use crate::config::Config;

pub struct Player {
    standing_image: Image,
    running_spritesheet: Image,
}

impl Player {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let standing_image = Image::new(context, &config.player_standing_image)?;
        let running_spritesheet = Image::new(context, &config.player_running_spritesheet)?;
        Ok(Player {
            standing_image,
            running_spritesheet,
        })
    }

    pub fn update(&mut self) -> GameResult {
        Ok(())
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        graphics::draw(
            context,
            &self.standing_image,
            DrawParam::new().scale([10.0, 10.0]),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use ggez::ContextBuilder;

    use crate::config;

    use super::*;

    #[test]
    fn test_new_player() {
        let config = config::load("config.json").unwrap();
        let (context, event_loop) =
            &mut match ContextBuilder::new("jungle", "Brooks Builds").build() {
                Ok((context, event_loop)) => (context, event_loop),
                Err(error) => panic!(error),
            };
        let player = Player::new(&config, context).unwrap();
    }
}
