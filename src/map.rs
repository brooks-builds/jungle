use ggez::{nalgebra::Point2, Context, GameResult};

use crate::{config::Config, game_objects::pit::Pit};

pub struct Map {
    current_index: usize,
}

impl Map {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let current_index = config.start_index;

        Ok(Map { current_index })
    }

    pub fn draw(&self, context: &mut Context, config: &Config) -> GameResult {
        if config.map[self.current_index].pits == 1 {
            self.center_pit.draw(config, context)?;
        }

        Ok(())
    }

    pub fn move_right(&mut self, config: &Config, context: &mut Context) -> GameResult {
        self.current_index = if self.current_index + 1 == config.map.len() {
            0
        } else {
            self.current_index + 1
        };
        Ok(())
    }

    pub fn move_left(&mut self, config: &Config, context: &mut Context) -> GameResult {
        self.current_index = if self.current_index == 0 {
            config.map.len() - 1
        } else {
            self.current_index - 1
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{config::load, initialize::initialize};

    use super::*;

    #[test]
    fn test_move_right() {
        let config = load("config.json").unwrap();
        let (context, _) = &mut initialize(&config).unwrap();
        let mut map = Map::new(&config, context).unwrap();
        map.move_right(&config, context).unwrap();
        assert_eq!(map.current_index, config.start_index + 1);
    }

    #[test]
    fn test_move_right_off_edge() {
        let config = load("config.json").unwrap();
        let (context, _) = &mut initialize(&config).unwrap();
        let mut map = Map::new(&config, context).unwrap();
        map.current_index = config.map.len() - 1;
        map.move_right(&config, context).unwrap();
        assert_eq!(map.current_index, 0);
    }

    #[test]
    fn test_move_left() {
        let config = load("config.json").unwrap();
        let (context, _) = &mut initialize(&config).unwrap();
        let mut map = Map::new(&config, context).unwrap();
        map.current_index = 1;
        map.move_left(&config, context).unwrap();
        assert_eq!(map.current_index, 0);
    }

    #[test]
    fn test_move_left_off_edge() {
        let config = load("config.json").unwrap();
        let (context, _) = &mut initialize(&config).unwrap();
        let mut map = Map::new(&config, context).unwrap();
        map.move_left(&config, context).unwrap();
        assert_eq!(map.current_index, config.map.len() - 1);
    }
}
