use std::{collections::HashMap, f32::consts::PI};

use ggez::{
    event::Button,
    graphics::DrawParam,
    graphics::{self, DrawMode, Font, Mesh, MeshBuilder, Rect, Scale, Text},
    nalgebra::Point2,
    Context, GameResult,
};
use rand::Rng;

use crate::{
    config::Config,
    game_objects::{
        bedrock::Bedrock, foliage::Foliage, ground::Ground, surface::Surface,
        surface_background::SurfaceBackground, tree_trunks::TreeTrunks, StaticGameObject,
    },
};

use super::Scene;

pub struct MainScene {
    map_index: usize,
    bedrock: Bedrock,
    ground: Ground,
    surface: Surface,
    surface_background: SurfaceBackground,
    tree_trunks: TreeTrunks,
    foliage: Foliage,
}

impl MainScene {
    pub fn new(config: &Config, context: &mut Context) -> GameResult<Self> {
        let map_index = config.start_index;
        let bedrock = Bedrock::new(config, context)?;
        let ground = Ground::new(config, context)?;
        let surface = Surface::new(config, context)?;
        let surface_background = SurfaceBackground::new(config, context)?;
        let tree_trunks = TreeTrunks::new(config, context)?;
        let foliage = Foliage::new(config, context)?;

        Ok(MainScene {
            map_index,
            bedrock,
            ground,
            surface,
            surface_background,
            tree_trunks,
            foliage,
        })
    }
}

impl Scene for MainScene {
    fn update(
        &mut self,
        _context: &mut Context,
        _button_pressed: Option<Button>,
        _config: &Config,
        _active_scene: &mut super::ActiveScene,
    ) -> GameResult {
        Ok(())
    }

    fn draw(&self, context: &mut Context, config: &Config) -> GameResult {
        self.bedrock.draw(config, context)?;
        self.ground.draw(config, context)?;
        self.surface.draw(config, context)?;
        self.surface_background.draw(config, context)?;
        self.tree_trunks.draw(config, context)?;
        self.foliage.draw(config, context)?;

        Ok(())
    }
}
