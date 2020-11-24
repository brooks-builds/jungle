use super::LifeSystem;

#[derive(Debug)]
pub struct PlayerLifeSystem {
    lives: u8,
}

impl PlayerLifeSystem {
    pub fn new(lives: u8) -> Self {
        Self { lives }
    }
}

impl LifeSystem for PlayerLifeSystem {
    fn get_lives(&self) -> u8 {
        self.lives
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ci_test_create_player_life_system() {
        let player_life_system: PlayerLifeSystem = PlayerLifeSystem::new(3);

        assert_eq!(player_life_system.lives, 3);
    }
}
