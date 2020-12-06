use ggez::graphics::DrawParam;

use super::DrawSystem;

pub struct TreeDrawSystem;

impl TreeDrawSystem {
    pub fn new() -> Self {
        Self
    }
}

impl DrawSystem for TreeDrawSystem {
    fn draw(
        &mut self,
        images: &mut crate::images::Images,
        config: &crate::config::Config,
        context: &mut ggez::Context,
        location: &ggez::nalgebra::Point2<f32>,
        physics_system: Option<crate::physics_systems::PhysicsState>,
        life_system: &Option<Box<dyn crate::life_systems::LifeSystem>>,
    ) -> ggez::GameResult {
        ggez::graphics::draw(context, &images.trees, DrawParam::new())?;
        ggez::graphics::draw(context, &images.foliage, DrawParam::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tree_draw_system() {
        let tree_draw_system: TreeDrawSystem = TreeDrawSystem::new();
    }
}
