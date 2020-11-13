pub mod config;
mod scenes;

use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{graphics, Context, GameResult};
use scenes::{
    end_scene::EndScene, main_scene::MainScene, pause_scene::PauseScene, start_scene::StartScene,
    ActiveScene, Scene,
};

#[derive(Default)]
pub struct GameState {
    active_scene: ActiveScene,
    starting_scene: StartScene,
    main_scene: MainScene,
    pause_scene: PauseScene,
    end_scene: EndScene,
}

impl GameState {
    pub fn new() -> GameState {
        let active_scene = ActiveScene::Start;
        let starting_scene = StartScene::new();
        let main_scene = MainScene::new();
        let pause_scene = PauseScene::new();
        let end_scene = EndScene::new();

        GameState {
            active_scene,
            starting_scene,
            main_scene,
            pause_scene,
            end_scene,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        match self.active_scene {
            ActiveScene::Start => self.starting_scene.update(context),
            ActiveScene::Main => self.main_scene.update(context),
            ActiveScene::Pause => self.pause_scene.update(context),
            ActiveScene::End => self.end_scene.update(context),
        }
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        match self.active_scene {
            ActiveScene::Start => self.starting_scene.draw(context)?,
            ActiveScene::Main => self.main_scene.draw(context)?,
            ActiveScene::Pause => self.pause_scene.draw(context)?,
            ActiveScene::End => self.end_scene.draw(context)?,
        }

        graphics::present(context)
    }
}
