use ggez::graphics;
use graphics::DrawParam;

use super::DrawSystem;

pub struct TreeDrawSystem {}

impl TreeDrawSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl DrawSystem for TreeDrawSystem {
    fn draw(
        &mut self,
        images: &mut crate::images::Images,
        _config: &crate::config::Config,
        context: &mut ggez::Context,
        _location: &ggez::nalgebra::Point2<f32>,
        _physics_system: Option<crate::physics_systems::PhysicsState>,
        _life_system: &Option<Box<dyn crate::life_systems::LifeSystem>>,
    ) -> ggez::GameResult {
        graphics::draw(context, &images.trees, DrawParam::new())
    }
}
