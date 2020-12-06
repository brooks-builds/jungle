use ggez::{
    graphics::DrawParam,
    graphics::{self, Font, Scale, Text},
    Context, GameResult,
};

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

    pub fn update(&mut self) -> GameResult {
        Ok(())
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.text, DrawParam::new())
    }
}
