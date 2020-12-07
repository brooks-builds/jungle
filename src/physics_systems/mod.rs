use ggez::nalgebra::Point2;

use crate::game_objects::GameObject;

pub mod player_physics_system;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PhysicsState {
    StandingStill,
    MovingRight,
    MovingLeft,
    Falling,
}

pub trait PhysicsSystem {
    fn update(
        &mut self,
        location: &mut Point2<f32>,
        command: Option<crate::handle_input::Command>,
        features: Vec<GameObject>,
    );
    fn get_state(&self) -> PhysicsState;
}
