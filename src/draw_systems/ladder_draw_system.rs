use ggez::graphics;
use graphics::DrawParam;

use super::DrawSystem;

pub struct LadderDrawSystem;

impl LadderDrawSystem {
    pub fn new() -> Self {
        Self
    }
}

impl DrawSystem for LadderDrawSystem {
    fn draw(
        &mut self,
        images: &mut crate::images::Images,
        _config: &crate::config::Config,
        context: &mut ggez::Context,
        _location: &ggez::nalgebra::Point2<f32>,
        _physics_system: Option<crate::physics_systems::PhysicsState>,
        _life_system: &Option<Box<dyn crate::life_systems::LifeSystem>>,
    ) -> ggez::GameResult {
        graphics::draw(context, &images.ladder, DrawParam::new())
    }
}

#[cfg(test)]
mod tests {
    use super::LadderDrawSystem;

    #[test]
    fn ci_test_create_ladder_draw_system() {
        let _ladder_draw_system: LadderDrawSystem = LadderDrawSystem::new();
    }
}
