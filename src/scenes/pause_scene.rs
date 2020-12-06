use ggez::{
    graphics::DrawParam,
    graphics::{self, Font, Scale, Text},
    Context, GameResult,
};
use graphics::Image;

use crate::{config::Config, handle_input::Command, images::Images};

use super::Scene;

#[derive(Default)]
pub struct PauseScene {
    text: Text,
}

impl PauseScene {
    pub fn new() -> Self {
        let mut text = Text::new("pause scene");
        text.set_font(Font::default(), Scale::uniform(72.0));

        PauseScene { text }
    }
}

impl Scene for PauseScene {
    fn update(
        &mut self,
        _context: &mut Context,
        _button_pressed: Option<Command>,
        _config: &Config,
        _active_scene: &mut super::ActiveScene,
        _images: &mut Images,
    ) -> GameResult {
        Ok(())
    }

    fn draw(
        &mut self,
        context: &mut Context,
        _config: &Config,
        _images: &mut Images,
    ) -> GameResult {
        graphics::draw(context, &self.text, DrawParam::new())
    }
}
