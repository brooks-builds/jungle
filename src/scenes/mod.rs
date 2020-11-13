use ggez::{Context, GameResult};

pub mod end_scene;
pub mod main_scene;
pub mod pause_scene;
pub mod start_scene;

#[allow(dead_code)]
pub enum ActiveScene {
    Start,
    Main,
    Pause,
    End,
}

impl Default for ActiveScene {
    fn default() -> Self {
        ActiveScene::Start
    }
}

pub trait Scene {
    fn update(&mut self, context: &mut Context) -> GameResult;

    fn draw(&self, context: &mut Context) -> GameResult;
}
