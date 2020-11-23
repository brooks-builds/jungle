use ggez::{nalgebra::Point2, Context, GameResult};

use crate::{
    config::Config,
    images::Images,
    physics_systems::{PhysicsState, PhysicsSystem},
};

pub mod player_draw_system;

pub trait DrawSystem {
    fn draw(
        &mut self,
        images: &Images,
        config: &Config,
        context: &mut Context,
        location: &Point2<f32>,
        physics_system: Option<PhysicsState>,
    ) -> GameResult;
}
