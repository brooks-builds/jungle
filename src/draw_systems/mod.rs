use ggez::{nalgebra::Point2, Context, GameResult};

use crate::{
    config::Config, images::Images, life_systems::LifeSystem, physics_systems::PhysicsState,
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
        life_system: &Option<Box<dyn LifeSystem>>,
    ) -> GameResult;
}