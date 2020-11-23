pub mod player_physics_system;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PhysicsState {
    StandingStill,
    MovingRight,
    MovingLeft,
}

pub trait PhysicsSystem {
    fn update(
        &mut self,
        location: &mut ggez::nalgebra::Point2<f32>,
        command: Option<crate::handle_input::Command>,
    );
    fn get_state(&self) -> PhysicsState;
}
