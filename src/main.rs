extern crate sdl2;

mod enemy_state;
mod game_state;
mod input_state;
mod math;
mod player_state;
mod projectile_state;
mod tile;

use crate::game_state::GameState;
use crate::input_state::InputState;

fn main() {
    println!("Hello, world!");

    // initialize window
    // initialize context
    // initialize controller

    let mut game_state = GameState::new();

    loop {
        let input_state = capture_input_state();

        // modify the game state based on the input
        game_state = input_system(game_state, &input_state);

        // we want physics first because we want the world to be in a valid state before
        // running the entity logic.
        // Example: a homing missle goes through a wall
        game_state = physics_system(game_state);
        game_state = entity_logic_system(game_state);

        render_system(&game_state);
        audio_system(&game_state);
    }
}

pub fn capture_input_state() -> InputState {
    InputState::new()
}

pub fn input_system(game_state: GameState, _input_state: &InputState) -> GameState {
    println!("input_system");
    game_state
}
pub fn physics_system(game_state: GameState) -> GameState {
    println!("physics_system");
    game_state
}
pub fn entity_logic_system(game_state: GameState) -> GameState {
    println!("entity_logic_system");
    game_state
}
pub fn render_system(_game_state: &GameState) -> () {
    println!("render_system");
}
pub fn audio_system(_game_state: &GameState) -> () {
    println!("audio_system");
}
