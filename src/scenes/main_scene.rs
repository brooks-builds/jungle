use ggez::{Context, GameResult};

use crate::config::MapFeature;
use crate::game_objects::builders::background::{
    create_above_ground, create_behind_ground, create_trees,
};
use crate::game_objects::builders::hearts::create_hearts;
use crate::game_objects::builders::ladder::create_ladder;
use crate::game_objects::builders::pit1::create_pit1;
use crate::game_objects::builders::player::create_player;
use crate::game_objects::{GameObjectTypes, GameObjects};
use crate::{config::Config, handle_input::Command, images::Images};

pub struct MainScene {
    game_objects: GameObjects,
    current_screen: usize,
}

impl MainScene {
    pub fn new(config: &Config, _context: &mut Context, images: &mut Images) -> GameResult<Self> {
        let mut game_objects = GameObjects::new();
        let player = create_player(config).expect("error creating player");
        let hearts = create_hearts(images, config).expect("error building hearts");

        game_objects.push(create_behind_ground(images, config).expect("error building "));
        game_objects.push(create_trees().expect("Error creating trees"));
        game_objects.push(hearts);
        game_objects.push(player);
        game_objects.push(
            create_above_ground(images.bedrock.clone(), config)
                .expect("error creating above background"),
        );

        let mut main_scene = MainScene {
            game_objects,
            current_screen: config.start_index,
        };

        main_scene.change_screen(config);

        Ok(main_scene)
    }

    pub fn update(
        &mut self,
        command: Option<Command>,
        config: &Config,
        images: &mut Images,
        context: &mut Context,
    ) -> GameResult {
        self.game_objects.update(command);

        if let Some(player) = self.game_objects.get_first_by_type(GameObjectTypes::Player) {
            if player.is_offscreen_right(config.resolution_x) {
                self.current_screen += 1;
                player.location.x = 0.0;
                self.change_screen(config);
                images.reset_trees(context, config)?;
            } else if player.is_offscreen_left() {
                self.current_screen -= 1;
                player.location.x = config.resolution_x;
                self.change_screen(config);
                images.reset_trees(context, config)?;
            }
        }

        Ok(())
    }

    pub fn draw(
        &mut self,
        context: &mut Context,
        config: &Config,
        images: &mut Images,
    ) -> GameResult {
        self.game_objects.draw(context, config, images)
    }

    fn change_screen(&mut self, config: &Config) {
        let player_index = self.game_objects.get_player_index().unwrap();
        self.game_objects.remove_features();
        config.map[self.current_screen]
            .iter()
            .for_each(|map_feature| match map_feature {
                MapFeature::Pit1 => {
                    let pit1 = create_pit1(config).expect("error creating pit1");
                    self.game_objects.insert(pit1, player_index);
                }
                MapFeature::Ladder => {
                    let ladder = create_ladder(config).expect("error creating ladder");
                    self.game_objects.push(ladder);
                }
            });
    }
}

#[cfg(test)]
mod test {
    use crate::{images::Images, initialize};

    use super::*;

    #[test]
    fn test_create_main_scene() {
        let config = crate::config::load("config.json").unwrap();
        let (context, _) = &mut initialize::initialize(&config).unwrap();
        let mut images = Images::new(context, &config).unwrap();
        let main_scene: MainScene = MainScene::new(&config, context, &mut images).unwrap();

        assert_eq!(main_scene.current_screen, config.start_index);
    }
}
