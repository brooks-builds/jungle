mod water_drop;

use ggez::{
    event::{EventHandler, KeyCode, KeyMods},
    graphics::{DrawParam, Font, Mesh, Scale, Text},
    input::gamepad::{
        gilrs::{self, EventType},
        Gilrs,
    },
    timer,
};
use ggez::{graphics, Context, GameResult};
use ggez::{
    graphics::{Color, DrawMode, MeshBuilder, BLACK},
    nalgebra::Point2,
};
use rand::{prelude::ThreadRng, Rng};
use water_drop::WaterDrop;

pub struct Game {
    water_drop_mesh: Mesh,
    water_drops: Vec<WaterDrop>,
    fps: Text,
    rng: ThreadRng,
    gilrs: Gilrs,
}

impl Game {
    pub fn new(context: &mut Context) -> GameResult<Game> {
        let water_drop_mesh = MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                Point2::new(0.0, 0.0),
                5.0,
                0.1,
                Color::from_rgb(67, 174, 199),
            )
            .build(context)?;
        Ok(Game {
            water_drop_mesh,
            water_drops: vec![],
            fps: Text::new("FPS Not Initialized..."),
            rng: rand::thread_rng(),
            gilrs: Gilrs::new()?,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.fps = Text::new(format!("FPS: {}", timer::fps(context)));
        self.fps.set_font(Font::default(), Scale::uniform(25.0));
        let (screen_width, screen_height) = graphics::drawable_size(context);
        self.water_drops
            .push(WaterDrop::new(self.rng.gen_range(0.0, screen_width), -10.0));
        while timer::check_update_time(context, 30) {
            self.water_drops
                .iter_mut()
                .for_each(|water_drop| water_drop.update());

            self.water_drops
                .retain(|water_drop| water_drop.is_alive(screen_height));
        }

        while let Some(event) = self.gilrs.next_event() {
            if let EventType::ButtonPressed(button, code) = event.event {
                dbg!(button as u16, code.into_u32());
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        graphics::draw(context, &self.fps, DrawParam::new().dest([5.0, 5.0]))?;

        self.water_drops
            .iter()
            .try_for_each(|water_drop| water_drop.draw(context, &self.water_drop_mesh))?;

        graphics::present(context)
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        println!("Keycode: {}", keycode as u32);
    }
}
