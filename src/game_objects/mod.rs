pub mod builders;
pub mod game_object;
pub mod game_object_types;

use ggez::{Context, GameResult};

use crate::config::Config;
use crate::handle_input::Command;
use crate::images::Images;

pub use self::game_object::GameObject;
pub use self::game_object_types::GameObjectTypes;

pub struct GameObjects {
    objects: Vec<GameObject>,
}

impl GameObjects {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn push(&mut self, game_object: GameObject) {
        self.objects.push(game_object);
    }

    pub fn get_first_by_type(
        &mut self,
        game_object_type: GameObjectTypes,
    ) -> Option<&mut GameObject> {
        self.objects
            .iter_mut()
            .find(|game_object| game_object.my_type == game_object_type)
    }

    pub fn update(&mut self, command: Option<Command>) {
        let features = self.get_all_features();
        self.objects
            .iter_mut()
            .for_each(|game_object| game_object.update(command, features.clone()));
    }

    pub fn draw(
        &mut self,
        context: &mut Context,
        config: &Config,
        images: &mut Images,
    ) -> GameResult {
        self.objects
            .iter_mut()
            .try_for_each(|game_object| game_object.draw(context, config, images))
    }

    pub fn remove_features(&mut self) {
        self.objects
            .retain(|game_object| game_object.my_type != GameObjectTypes::Feature);
    }

    pub fn remove_player(&mut self) -> Option<GameObject> {
        let index = self
            .objects
            .iter()
            .enumerate()
            .find(|(_index, game_object)| game_object.my_type == GameObjectTypes::Player);

        if let Some((player_index, _)) = index {
            Some(self.objects.remove(player_index))
        } else {
            None
        }
    }

    pub fn get_all_features(&self) -> Vec<GameObject> {
        self.objects
            .clone()
            .into_iter()
            .filter(|game_object| game_object.my_type == GameObjectTypes::Feature)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use builders::pit1::create_pit1;
    use builders::player::create_player;
    use game_object::GameObjectBuilder;

    use crate::config;
    use crate::draw_systems::single_pit_draw_system::SinglePitDrawSystem;
    use crate::initialize::initialize;

    use super::*;

    #[test]
    fn ci_test_creating_game_objects() {
        let game_objects: GameObjects = GameObjects::new();

        assert_eq!(game_objects.objects.len(), 0);
    }

    #[test]
    fn ci_test_inserting_game_object() {
        let mut game_objects = GameObjects::new();
        let basic_game_object = GameObjectBuilder::new()
            .draw_system(Box::new(SinglePitDrawSystem::new()))
            .location(ggez::nalgebra::Point2::new(0.0, 0.0))
            .width(1.0)
            .with_type(GameObjectTypes::Player)
            .build()
            .unwrap();
        game_objects.push(basic_game_object);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_get_first_game_object_by_type() {
        let mut game_objects = GameObjects::new();
        let config = config::load("config.json").unwrap();
        let player = create_player(&config).unwrap();
        game_objects.push(player);

        if let Some(player) = game_objects.get_first_by_type(GameObjectTypes::Player) {
            assert_eq!(player.location.x, config.player_starting_x);
        }
    }

    #[test]
    fn ci_test_update_game_objects() {
        let config = config::load("config.json").unwrap();
        let player = create_player(&config).unwrap();
        let mut game_objects = GameObjects::new();
        let command = Command::MoveRight;
        game_objects.push(player);
        game_objects.update(Some(command));
    }

    #[test]
    fn test_draw_game_objects() {
        let mut game_objects = GameObjects::new();
        let config = config::load("config.json").unwrap();
        let player = create_player(&config).unwrap();
        let (context, _) = &mut initialize(&config).unwrap();
        let mut images = Images::new(context, &config).unwrap();

        game_objects.push(player);

        game_objects.draw(context, &config, &mut images).unwrap();
    }

    #[test]
    fn ci_test_remove_game_features() {
        let config = config::load("config.json").unwrap();
        let player = create_player(&config).unwrap();
        let mut game_objects = GameObjects::new();
        let pit = create_pit1(&config).unwrap();

        game_objects.push(player);
        game_objects.push(pit);

        assert_eq!(game_objects.objects.len(), 2);
        game_objects.remove_features();
        assert_eq!(game_objects.objects.len(), 1);
        assert_eq!(game_objects.objects[0].my_type, GameObjectTypes::Player);
    }

    #[test]
    fn ci_test_remove_player() {
        let config = config::load("config.json").unwrap();
        let player = create_player(&config).unwrap();
        let mut game_objects = GameObjects::new();

        game_objects.push(player);

        let player = game_objects.remove_player().unwrap();
        assert_eq!(player.my_type, GameObjectTypes::Player);
        assert_eq!(game_objects.objects.len(), 0);
    }

    #[test]
    fn ci_test_get_all_features() {
        let mut game_objects = GameObjects::new();
        let config = &config::load("config.json").unwrap();
        let pit = create_pit1(config).unwrap();
        game_objects.push(pit);

        let all_features: Vec<GameObject> = game_objects.get_all_features();
        assert_eq!(all_features.len(), 1);
        assert_eq!(all_features[0].my_type, GameObjectTypes::Feature);
    }
}
