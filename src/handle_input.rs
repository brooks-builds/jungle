use ggez::{event::Button, input::gamepad::Gilrs, GameResult};

use crate::{config::Config, scenes::ActiveScene};

#[derive(Debug, PartialEq)]
pub enum Command {
    StartGame,
    MoveRight,
    StopMovingRight,
    MoveLeft,
    StopMovingLeft,
}

impl Command {
    pub fn stop(&mut self) {
        match self {
            Command::MoveRight => *self = Command::StopMovingRight,
            Command::StartGame => {}
            Command::StopMovingRight => {}
            Command::MoveLeft => *self = Command::StopMovingLeft,
            Command::StopMovingLeft => {}
        }
    }
}

pub struct HandleInput {
    gamepad: Gilrs,
    start_button: Button,
    move_right: Button,
    move_left: Button,
}

impl HandleInput {
    pub fn new(config: &Config) -> GameResult<Self> {
        let gamepad = Gilrs::new()?;
        let start_button = config.start_button;
        let move_right = config.move_right_button;
        let move_left = config.move_left_button;

        Ok(Self {
            gamepad,
            start_button,
            move_right,
            move_left,
        })
    }

    pub fn run(&mut self, current_scene: &ActiveScene) -> Option<Command> {
        if let Some(gamepad_event) = self.gamepad.next_event() {
            match gamepad_event.event {
                ggez::input::gamepad::gilrs::EventType::ButtonRepeated(button, _code) => {
                    dbg!("hello", button);
                    None
                }
                ggez::input::gamepad::gilrs::EventType::ButtonPressed(button, _code) => {
                    self.button_to_command(button, current_scene)
                }
                ggez::input::gamepad::gilrs::EventType::ButtonReleased(button, _) => {
                    let mut command = self.button_to_command(button, current_scene);
                    if let Some(command) = &mut command {
                        command.stop();
                    }

                    command
                }
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
            (button, ActiveScene::Main) if button == self.move_left => Some(Command::MoveLeft),
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

    #[test]
    fn ci_test_stop_moving_when_moving() {
        let mut moving = Command::MoveRight;
        moving.stop();
        assert_eq!(moving, Command::StopMovingRight);
    }

    #[test]
    fn ci_test_stopping_when_stopped_doesnt_do_anything() {
        let mut stopped_moving = Command::StopMovingRight;
        stopped_moving.stop();
        assert_eq!(stopped_moving, Command::StopMovingRight);
    }

    #[test]
    fn ci_test_left_button_while_not_moving() {
        let scene = ActiveScene::Main;
        let config = config::load("config.json").unwrap();
        let handle_input = HandleInput::new(&config).unwrap();
        let command = handle_input
            .button_to_command(config.move_left_button, &scene)
            .unwrap();
        assert_eq!(command, Command::MoveLeft);
    }

    #[test]
    fn ci_test_stopping_left_button_while_moving() {
        let mut command = Command::MoveLeft;
        command.stop();
        assert_eq!(command, Command::StopMovingLeft);
    }
}
