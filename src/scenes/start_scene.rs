use ggez::{
    event::Button,
    graphics::DrawParam,
    graphics::{self, Font, Scale, Text},
    Context, GameResult,
};

use crate::config::Config;

use super::Scene;

#[derive(Default)]
pub struct StartScene {
    text: Text,
}

impl StartScene {
    pub fn new() -> Self {
        let mut text = Text::new("Start Scene");
        text.set_font(Font::default(), Scale::uniform(72.0));

        StartScene { text }
    }
}

impl Scene for StartScene {
    fn update(
        &mut self,
        _context: &mut Context,
        button_pressed: Option<Button>,
        config: &Config,
        active_scene: &mut super::ActiveScene,
    ) -> GameResult {
        if let Some(button) = button_pressed {
            if button as u16 == config.start_button as u16 {
                active_scene.change_to_main();
            }
        }
        Ok(())
    }

    fn draw(&self, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.text, DrawParam::new())
    }
}
