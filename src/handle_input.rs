use ggez::{event::Button, input::gamepad::Gilrs, GameResult};

use crate::{config::Config, scenes::ActiveScene};

#[derive(Debug, PartialEq)]
pub enum Command {
    StartGame,
    MoveRight,
}

pub struct HandleInput {
    gamepad: Gilrs,
    start_button: Button,
    move_right: Button,
}

impl HandleInput {
    pub fn new(config: &Config) -> GameResult<Self> {
        let gamepad = Gilrs::new()?;
        let start_button = config.start_button.clone();
        let move_right = config.move_right_button.clone();

        Ok(Self {
            gamepad,
            start_button,
            move_right,
        })
    }

    pub fn run(&mut self, current_scene: &ActiveScene) -> Option<Command> {
        if let Some(gamepad_event) = self.gamepad.next_event() {
            match gamepad_event.event {
                ggez::input::gamepad::gilrs::EventType::ButtonRepeated(button, code) => {
                    dbg!("hello", button);
                    None
                }
                ggez::input::gamepad::gilrs::EventType::ButtonPressed(button, code) => {
                    self.button_to_command(button, current_scene)
                }
                ggez::input::gamepad::gilrs::EventType::ButtonReleased(button, _) => None,
                ggez::input::gamepad::gilrs::EventType::ButtonChanged(_, _, _) => None,
                ggez::input::gamepad::gilrs::EventType::AxisChanged(_, _, _) => None,
                ggez::input::gamepad::gilrs::EventType::Connected => None,
                ggez::input::gamepad::gilrs::EventType::Disconnected => None,
                ggez::input::gamepad::gilrs::EventType::Dropped => None,
            }
        } else {
            None
        }
    }

    fn button_to_command(&self, button: Button, current_scene: &ActiveScene) -> Option<Command> {
        match (button, current_scene) {
            (button, ActiveScene::Start) if button == self.start_button => Some(Command::StartGame),
            (button, ActiveScene::Main) if button == self.move_right => Some(Command::MoveRight),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{config, scenes::ActiveScene};

    use super::*;

    #[test]
    fn ci_test_create_handle_input() {
        let config = config::load("config.json").unwrap();
        let handle_input = HandleInput::new(&config).unwrap();
        assert_eq!(handle_input.start_button, config.start_button);
    }

    #[test]
    fn ci_test_handle_input_start_game() {
        let start_scene = ActiveScene::Start;
        let config = config::load("config.json").unwrap();
        let handle_input = HandleInput::new(&config).unwrap();
        let command = handle_input
            .button_to_command(config.start_button, &start_scene)
            .unwrap();
        assert_eq!(command, Command::StartGame);
    }

    #[test]
    fn ci_test_handle_input_start_game_while_ended() {
        let scene = ActiveScene::End;
        let config = config::load("config.json").unwrap();
        let handle_input = HandleInput::new(&config).unwrap();
        let command = handle_input.button_to_command(config.start_button, &scene);
        assert_eq!(command, None);
    }

    #[test]
    fn ci_test_right_button_while_stopped() {
        let scene = ActiveScene::Main;
        let config = config::load("config.json").unwrap();
        let handle_input = HandleInput::new(&config).unwrap();
        let command = handle_input
            .button_to_command(config.move_right_button, &scene)
            .unwrap();
        assert_eq!(command, Command::MoveRight);
    }
}
