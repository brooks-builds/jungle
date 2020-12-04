use ggez::conf::Backend;
use ggez::{
    conf::FullscreenType, conf::WindowMode, event::EventsLoop, Context, ContextBuilder, GameResult,
};

use crate::config::Config;

pub fn initialize(config: &Config) -> GameResult<(Context, EventsLoop)> {
    let backend = Backend::default().version(3, 1).gles();
    let window_mode = WindowMode::default()
        .dimensions(config.resolution_x, config.resolution_y)
        .fullscreen_type(FullscreenType::True);
    ContextBuilder::new("jungle", "Brooks Builds")
        .window_mode(window_mode)
        .backend(backend)
        .build()
}

#[cfg(test)]
mod test {
    use crate::config;

    use super::*;

    #[test]
    fn test_initializing_game() {
        let config = config::load("config.json").unwrap();
        let (_context, _event_loop) = &mut initialize(&config).unwrap();
    }
}
