use ggez::graphics::DrawParam;

use super::DrawSystem;

pub struct SinglePitDrawSystem;

impl SinglePitDrawSystem {
    pub fn new() -> Self {
        Self
    }
}

impl DrawSystem for SinglePitDrawSystem {
    fn draw(
        &mut self,
        images: &mut crate::images::Images,
        _config: &crate::config::Config,
        context: &mut ggez::Context,
        location: &ggez::nalgebra::Point2<f32>,
        _physics_system: Option<crate::physics_systems::PhysicsState>,
        _life_system: &Option<Box<dyn crate::life_systems::LifeSystem>>,
    ) -> ggez::GameResult {
        ggez::graphics::draw(
            context,
            &images.pit1,
            DrawParam::new().dest([location.x, location.y]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ci_test_create_single_pit_draw_system() {
        let _pit_draw_system: SinglePitDrawSystem = SinglePitDrawSystem::new();
    }
}
