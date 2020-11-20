use ggez::{nalgebra::Point2, Context, GameResult};

use crate::{config::Config, images::Images};

pub mod player_draw_system;

pub trait DrawSystem {
    fn draw(
        &self,
        images: &Images,
        config: &Config,
        context: &mut Context,
        location: &Point2<f32>,
    ) -> GameResult;
}
