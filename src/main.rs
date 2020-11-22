use ggez::event;
use jungle::{config, initialize::initialize, GameState};

fn main() {
    let config = config::load("config.json").unwrap();
    let (context, event_loop) = &mut initialize(&config).unwrap();

    let game_state = &mut GameState::new(config, context).unwrap();

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };
}
