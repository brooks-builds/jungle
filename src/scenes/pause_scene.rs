use ggez::{
    graphics::DrawParam,
    graphics::{self, Font, Scale, Text},
    Context, GameResult,
};

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

    pub fn update(&mut self) -> GameResult {
        Ok(())
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.text, DrawParam::new())
    }
}
