pub mod config;
mod draw_systems;
mod game_objects;
mod handle_input;
mod images;
pub mod initialize;
mod life_systems;
mod physics_systems;
mod scenes;

use config::Config;
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};
use ggez::{graphics::BLACK, timer};
use handle_input::HandleInput;
use images::Images;
use scenes::{
    end_scene::EndScene, main_scene::MainScene, pause_scene::PauseScene, start_scene::StartScene,
    ActiveScene,
};

pub struct GameState {
    active_scene: ActiveScene,
    starting_scene: StartScene,
    main_scene: MainScene,
    pause_scene: PauseScene,
    end_scene: EndScene,
    handle_input: HandleInput,
    config: Config,
    images: Images,
}

impl GameState {
    pub fn new(config: Config, context: &mut Context) -> GameResult<Self> {
        let active_scene = ActiveScene::Start;
        let starting_scene = StartScene::new(&config, context);
        let mut images = Images::new(context, &config)?;
        let main_scene = MainScene::new(&config, context, &mut images)?;
        let pause_scene = PauseScene::new();
        let end_scene = EndScene::new();
        let handle_input = HandleInput::new(&config)?;

        Ok(Self {
            active_scene,
            starting_scene,
            main_scene,
            pause_scene,
            end_scene,
            handle_input,
            config,
            images,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, 30) {
            let command = self.handle_input.run(&self.active_scene);

            match self.active_scene {
                ActiveScene::Start => self
                    .starting_scene
                    .update(command, &mut self.active_scene)?,
                ActiveScene::Main => {
                    self.main_scene
                        .update(command, &self.config, &mut self.images, context)?
                }
                ActiveScene::Pause => self.pause_scene.update()?,
                ActiveScene::End => self.end_scene.update()?,
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        match self.active_scene {
            ActiveScene::Start => self.starting_scene.draw(context)?,
            ActiveScene::Main => self
                .main_scene
                .draw(context, &self.config, &mut self.images)?,
            ActiveScene::Pause => self.pause_scene.draw(context)?,
            ActiveScene::End => self.end_scene.draw(context)?,
        }

        graphics::present(context)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_game_state() {
        let config = config::load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let game_state = GameState::new(config, context).unwrap();

        assert_eq!(game_state.active_scene, ActiveScene::Start);
    }
}
