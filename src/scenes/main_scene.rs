use ggez::{
    graphics::DrawParam,
    graphics::{self, Font, Scale, Text},
    Context, GameResult,
};

use super::Scene;

#[derive(Default)]
pub struct MainScene {
    text: Text,
}

impl MainScene {
    pub fn new() -> Self {
        let mut text = Text::new("main scene");
        text.set_font(Font::default(), Scale::uniform(72.0));

        MainScene { text }
    }
}

impl Scene for MainScene {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&self, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.text, DrawParam::new())
    }
}
