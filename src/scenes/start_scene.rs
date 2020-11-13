use ggez::{
    graphics::DrawParam,
    graphics::{self, Font, Scale, Text},
    Context, GameResult,
};

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
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&self, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.text, DrawParam::new())
    }
}
