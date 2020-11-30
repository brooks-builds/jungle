use super::DrawSystem;

pub struct HeartDrawSystem {}

impl HeartDrawSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl DrawSystem for HeartDrawSystem {
    fn draw(
        &mut self,
        images: &crate::images::Images,
        config: &crate::config::Config,
        context: &mut ggez::Context,
        location: &ggez::nalgebra::Point2<f32>,
        physics_system: Option<crate::physics_systems::PhysicsState>,
        life_system: &Option<Box<dyn crate::life_systems::LifeSystem>>,
    ) -> ggez::GameResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ci_test_create_heart_draw_system() {
        let heart_draw_system = HeartDrawSystem::new();
    }
}
