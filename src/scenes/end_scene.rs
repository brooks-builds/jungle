use ggez::{
    event::Button,
    graphics::DrawParam,
    graphics::{self, Font, Scale, Text},
    Context, GameResult,
};

use crate::{config::Config, handle_input::Command, images::Images};

use super::Scene;

#[derive(Default)]
pub struct EndScene {
    text: Text,
}

impl EndScene {
    pub fn new() -> Self {
        let mut text = Text::new("end scene");
        text.set_font(Font::default(), Scale::uniform(72.0));

        EndScene { text }
    }
}

impl Scene for EndScene {
    fn update(
        &mut self,
        _context: &mut Context,
        _button_pressed: Option<Command>,
        _config: &Config,
        _active_scene: &mut super::ActiveScene,
    ) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context, _config: &Config, _images: &Images) -> GameResult {
        graphics::draw(context, &self.text, DrawParam::new())
    }
}
