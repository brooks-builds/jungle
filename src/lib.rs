pub mod config;
mod scenes;

use config::Config;
use ggez::{event::EventHandler, input::gamepad::Gilrs};
use ggez::{graphics, Context, GameResult};
use ggez::{graphics::BLACK, timer};
use scenes::{
    end_scene::EndScene, main_scene::MainScene, pause_scene::PauseScene, start_scene::StartScene,
    ActiveScene, Scene,
};

pub struct GameState {
    active_scene: ActiveScene,
    starting_scene: StartScene,
    main_scene: MainScene,
    pause_scene: PauseScene,
    end_scene: EndScene,
    gamepad: Gilrs,
    config: Config,
}

impl GameState {
    pub fn new(config: Config) -> GameResult<GameState> {
        let active_scene = ActiveScene::Start;
        let starting_scene = StartScene::new();
        let main_scene = MainScene::new();
        let pause_scene = PauseScene::new();
        let end_scene = EndScene::new();
        let gamepad = Gilrs::new()?;

        Ok(GameState {
            active_scene,
            starting_scene,
            main_scene,
            pause_scene,
            end_scene,
            gamepad,
            config,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, 30) {
            let button_pressed = if let Some(gamepad_event) = self.gamepad.next_event() {
                match gamepad_event.event {
                    ggez::input::gamepad::gilrs::EventType::ButtonPressed(button, code) => {
                        Some(button)
                    }
                    _ => None,
                }
            } else {
                None
            };

            match self.active_scene {
                ActiveScene::Start => self.starting_scene.update(
                    context,
                    button_pressed,
                    &self.config,
                    &mut self.active_scene,
                )?,
                ActiveScene::Main => self.main_scene.update(
                    context,
                    button_pressed,
                    &self.config,
                    &mut self.active_scene,
                )?,
                ActiveScene::Pause => self.pause_scene.update(
                    context,
                    button_pressed,
                    &self.config,
                    &mut self.active_scene,
                )?,
                ActiveScene::End => self.end_scene.update(
                    context,
                    button_pressed,
                    &self.config,
                    &mut self.active_scene,
                )?,
            }
        }

        Ok(())
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
