use ggez::{nalgebra::Point2, Context, GameResult};

use crate::{
    config::Config,
    game_objects::{
        bedrock::Bedrock, foliage::Foliage, ground::Ground, pit::Pit, surface::Surface,
        surface_background::SurfaceBackground, tree_trunks::TreeTrunks, StaticGameObject,
    },
};

pub struct Map {
    current_index: usize,
    bedrock: Bedrock,
    ground: Ground,
    surface: Surface,
    surface_background: SurfaceBackground,
    tree_trunks: TreeTrunks,
    foliage: Foliage,
    center_pit: Pit,
}

impl Map {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let current_index = config.start_index;
        let bedrock = Bedrock::new(config, context)?;
        let ground = Ground::new(config, context)?;
        let surface = Surface::new(config, context)?;
        let surface_background = SurfaceBackground::new(config, context)?;
        let tree_trunks = TreeTrunks::new(config, context)?;
        let foliage = Foliage::new(config, context)?;
        let center_pit = Pit::new(
            config,
            context,
            Point2::new(
                config.resolution_x / 2.0 - config.pit_width / 2.0,
                config.resolution_y
                    - config.bedrock_height
                    - config.cave_height
                    - config.ground_height
                    - config.surface_height
                    + config.pit_margin,
            ),
        )?;

        Ok(Map {
            current_index,
            bedrock,
            ground,
            surface,
            surface_background,
            tree_trunks,
            foliage,
            center_pit,
        })
    }

    pub fn draw(&self, context: &mut Context, config: &Config) -> GameResult {
        self.bedrock.draw(config, context)?;
        self.ground.draw(config, context)?;
        self.surface.draw(config, context)?;
        self.surface_background.draw(config, context)?;
        self.tree_trunks.draw(config, context)?;
        self.foliage.draw(config, context)?;

        if config.map[self.current_index].pits == 1 {
            self.center_pit.draw(config, context)?;
        }

        Ok(())
    }
}
