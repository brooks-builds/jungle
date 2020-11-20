use ggez::{
    graphics::{self, DrawParam},
    nalgebra::Point2,
    Context, GameResult,
};

use crate::{config::Config, images::Images};

use super::DrawSystem;

pub struct PlayerDrawSystem {}

impl PlayerDrawSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl DrawSystem for PlayerDrawSystem {
    fn draw(
        &self,
        images: &Images,
        _config: &Config,
        context: &mut Context,
        location: &Point2<f32>,
    ) -> GameResult {
        graphics::draw(
            context,
            &images.standing_player,
            DrawParam::new().dest([location.x, location.y]),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ci_test_create_player_draw_system() {
        let player_draw_system: PlayerDrawSystem = PlayerDrawSystem::new();
        let _boxed_player_draw_system: Box<dyn DrawSystem> = Box::new(player_draw_system);
    }
}
