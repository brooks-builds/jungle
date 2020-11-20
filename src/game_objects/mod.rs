pub mod bedrock;
pub mod foliage;
pub mod ground;
pub mod pit;
pub mod surface;
pub mod surface_background;
pub mod tree_trunks;

use ggez::{nalgebra::Point2, Context, GameResult};

use crate::{config::Config, draw_systems::DrawSystem, images::Images};

pub trait StaticGameObject {
    fn draw(&self, config: &Config, context: &mut Context) -> GameResult;
}

pub struct GameObject {
    location: Point2<f32>,
    draw_system: Box<dyn DrawSystem>,
}

impl GameObject {
    pub fn new(location: Point2<f32>, draw_system: Box<dyn DrawSystem>) -> GameResult<Self> {
        Ok(Self {
            location,
            draw_system,
        })
    }

    pub fn draw(&self, context: &mut Context, config: &Config, images: &Images) -> GameResult {
        self.draw_system
            .draw(images, config, context, &self.location)
    }
}

#[cfg(test)]
mod test {
    use ggez::nalgebra::Point2;

    use crate::{draw_systems::player_draw_system::PlayerDrawSystem, images::Images, initialize};

    use super::*;

    #[test]
    fn ci_test_create_empty_game_object() {
        let location: Point2<f32> = Point2::new(10.0, 10.0);
        let player_draw_system = PlayerDrawSystem::new();
        let game_object: GameObject =
            GameObject::new(location, Box::new(player_draw_system)).unwrap();
        assert_eq!(game_object.location, location);
    }

    #[test]
    fn test_draw_game_object() {
        let location: Point2<f32> = Point2::new(10.0, 10.0);
        let player_draw_system = PlayerDrawSystem::new();
        let game_object: GameObject =
            GameObject::new(location, Box::new(player_draw_system)).unwrap();
        let config = crate::config::load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let images = Images::new(context, &config).unwrap();
        game_object.draw(context, &config, &images).unwrap();
    }
}
