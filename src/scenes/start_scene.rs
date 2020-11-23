use ggez::{
    graphics::DrawParam,
    graphics::{self, Font, Scale, Text},
    nalgebra::Point2,
    Context, GameResult,
};

use crate::{config::Config, handle_input::Command, images::Images};

use super::Scene;

pub struct StartScene {
    title: Text,
    title_position: Point2<f32>,
    subtitle: Text,
    subtitle_position: Point2<f32>,
}

impl StartScene {
    pub fn new(config: &Config, context: &mut Context) -> Self {
        let mut title = Text::new(config.title.clone());
        title.set_font(Font::default(), Scale::uniform(config.font_large));
        let (title_width, title_height) = title.dimensions(context);
        let title_position = Point2::new(
            config.resolution_x / 2.0 - (title_width / 2) as f32,
            config.resolution_y / 2.0 - (title_height / 2) as f32,
        );

        let mut subtitle = Text::new(config.title_subtext.clone());
        subtitle.set_font(Font::default(), Scale::uniform(config.font_small));
        let subtitle_position = Point2::new(
            config.resolution_x / 2.0 - subtitle.width(context) as f32 / 2.0,
            config.resolution_y - config.resolution_y / 4.0,
        );

        StartScene {
            title,
            title_position,
            subtitle,
            subtitle_position,
        }
    }
}

impl Scene for StartScene {
    fn update(
        &mut self,
        _context: &mut Context,
        command: Option<Command>,
        _config: &Config,
        active_scene: &mut super::ActiveScene,
    ) -> GameResult {
        if let Some(command) = command {
            if matches!(command, Command::StartGame) {
                active_scene.change_to_main();
            }
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context, _config: &Config, _images: &Images) -> GameResult {
        graphics::draw(
            context,
            &self.title,
            DrawParam::new().dest(self.title_position),
        )?;

        graphics::draw(
            context,
            &self.subtitle,
            DrawParam::new().dest(self.subtitle_position),
        )
    }
}
