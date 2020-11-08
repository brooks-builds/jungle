use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    event, ContextBuilder,
};
use ggez_demo::Game;

fn main() {
    let window_mode = WindowMode::default().dimensions(1920.0, 1080.0);
    // .fullscreen_type(FullscreenType::True);
    let window_setup = WindowSetup::default().vsync(false);
    let (context, event_loop) = &mut match ContextBuilder::new("GGEZ_demo", "Brookzerker")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
    {
        Ok((context, event_loop)) => (context, event_loop),
        Err(error) => panic!(error),
    };

    let mut game_state = match Game::new(context) {
        Ok(game_state) => game_state,
        Err(error) => {
            eprintln!("error creating game state: {}", error);
            panic!();
        }
    };

    match event::run(context, event_loop, &mut game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };
}
