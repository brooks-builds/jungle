pub mod player_physics_system;

pub trait PhysicsSystem {
    fn update(
        &mut self,
        location: &mut ggez::nalgebra::Point2<f32>,
        command: Option<crate::handle_input::Command>,
    );
}
