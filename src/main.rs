use ggez::conf::{FullscreenType, WindowMode};
use ggez::{event, ContextBuilder};
use jungle::{config, GameState};

fn main() {
    let config = config::load("config.json").unwrap();
    let window_mode = WindowMode::default()
        .dimensions(config.resolution_x, config.resolution_y)
        .fullscreen_type(FullscreenType::True);
    let (context, event_loop) = &mut match ContextBuilder::new("jungle", "Brooks Builds")
        .window_mode(window_mode)
        .build()
    {
        Ok((context, event_loop)) => (context, event_loop),
        Err(error) => panic!(error),
    };

    let game_state = &mut GameState::new(config, context).unwrap();

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };
}
