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

fn main() -> Result<(), String> {
    println!("Hello, world!");

    // initialize sdl
    let sdl_context = sdl2::init()?;

    // initialize controller - taken from rust-sdl2 example
    let game_controller_subsystem = sdl_context.game_controller()?;
    let num_joysticks_available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;
    // Iterate over all available joysticks and look for game controllers.
    let mut controller = (0..num_joysticks_available)
        .find_map(|id| {
            if !game_controller_subsystem.is_game_controller(id) {
                println!("{} is not a game controller", id);
                return None;
            }

            println!("Attempting to open controller {}", id);

            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    println!("Success: opened \"{}\"", c.name());
                    Some(c)
                }
                Err(e) => {
                    println!("failed: {:?}", e);
                    None
                }
            }
        })
        .expect("Couldn't open any controller");

    // initialize window
    // initialize context

    let mut game_state = GameState::new();

    loop {
        let mut input_state = InputState::new();
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            use sdl2::controller::Axis;
            use sdl2::event::Event;
            if let Event::ControllerAxisMotion {
                axis, value: val, ..
            } = event
            {
                let dead_zone = 10_000;
                if val >= -dead_zone && val <= dead_zone {
                    continue;
                }
                let normalized = if val < 0 {
                    println!("{}", val);
                    println!("{}", i16::min_value());
                    -(val as f32 / i16::min_value() as f32)
                } else {
                    val as f32 / i16::max_value() as f32
                };

                match axis {
                    Axis::LeftX => input_state.ls.x = normalized,
                    Axis::LeftY => input_state.ls.y = normalized,
                    Axis::RightX => input_state.rs.x = normalized,
                    Axis::RightY => input_state.rs.y = normalized,
                    Axis::TriggerRight => (),
                    Axis::TriggerLeft => (),
                }
            }
        }
        input_state.ls.normalize();
        input_state.rs.normalize();
        println!("input: {:?}", input_state);
        //let input_state = capture_input_state();

        // modify the game state based on the input
        game_state = input_system(game_state, &input_state);

        // we want physics first because we want the world to be in a valid state before
        // running the entity logic.
        // Example: a homing missle goes through a wall
        game_state = physics_system(game_state);
        game_state = entity_logic_system(game_state);

        render_system(&game_state);
        audio_system(&game_state);

        // wait for next frame
        let sleep_dur = std::time::Duration::from_millis(500);
        std::thread::sleep(sleep_dur);
        println!("end_frame");
    }
    Ok(())
}

pub fn capture_input_state() -> InputState {
    InputState::new()
}

pub fn input_system(game_state: GameState, _input_state: &InputState) -> GameState {
    // println!("input_system");
    game_state
}
pub fn physics_system(game_state: GameState) -> GameState {
    // println!("physics_system");
    game_state
}
pub fn entity_logic_system(game_state: GameState) -> GameState {
    // println!("entity_logic_system");
    game_state
}
pub fn render_system(_game_state: &GameState) -> () {
    // println!("render_system");
}
pub fn audio_system(_game_state: &GameState) -> () {
    // println!("audio_system");
}
