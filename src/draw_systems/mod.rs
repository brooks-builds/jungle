use ggez::{nalgebra::Point2, Context, GameResult};

use crate::{
    config::Config, images::Images, life_systems::LifeSystem, physics_systems::PhysicsState,
};

pub mod background_draw_system;
pub mod hearts_draw_system;
pub mod player_draw_system;
pub mod single_pit_draw_system;

pub trait DrawSystem {
    fn draw(
        &mut self,
        images: &mut Images,
        config: &Config,
        context: &mut Context,
        location: &Point2<f32>,
        physics_system: Option<PhysicsState>,
        life_system: &Option<Box<dyn LifeSystem>>,
    ) -> GameResult;
}
