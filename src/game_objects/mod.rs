pub mod bedrock;
pub mod foliage;
pub mod ground;
pub mod pit;
pub mod player;
pub mod surface;
pub mod surface_background;
pub mod tree_trunks;

use ggez::{Context, GameResult};

use crate::config::Config;

pub trait StaticGameObject {
    fn draw(&self, config: &Config, context: &mut Context) -> GameResult;
}
