use ggez::{event::Button, Context, GameResult};

use crate::config::Config;

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

impl ActiveScene {
    pub fn change_to_main(&mut self) {
        *self = ActiveScene::Main;
    }
}

impl Default for ActiveScene {
    fn default() -> Self {
        ActiveScene::Start
    }
}

pub trait Scene {
    fn update(
        &mut self,
        context: &mut Context,
        button_pressed: Option<Button>,
        config: &Config,
        active_scene: &mut ActiveScene,
    ) -> GameResult;

    fn draw(&self, context: &mut Context) -> GameResult;
}
