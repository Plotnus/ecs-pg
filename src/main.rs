#![feature(clamp)]
#![feature(duration_float)]
extern crate sdl2;

mod enemy_state;
mod game_state;
mod input_state;
mod math;
mod player_state;
mod projectile_state;
mod tile;

use crate::game_state::GameState;

// used for input
use input_state::CoreControllerInput;
use input_state::GameControllerInput;
use input_state::RawControllerInput;
use math::Vec2;
use math::Vec3;

fn main() -> Result<(), String> {
    println!("Hello, world!");

    // initialize sdl
    let sdl_context = sdl2::init()?;

    // initialize controller - taken from rust-sdl2 example
    let controller_subsystem = sdl_context.game_controller()?;
    let num_joysticks_available = controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;
    // Iterate over all available joysticks and look for a game controller.
    let controller = (0..num_joysticks_available)
        .find_map(|id| {
            if !controller_subsystem.is_game_controller(id) {
                println!("{} is not a game controller", id);
                return None;
            }

            println!("Attempting to open controller {}", id);

            match controller_subsystem.open(id) {
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

    let timer = sdl_context.timer()?;
    let mut game_state = GameState::new();

    loop {
        let input_state = {
            // because we are not using the event loop we must call update on the controller subsystem
            controller_subsystem.update();
            let raw_controller_input = RawControllerInput::from_sdl2_controller(&controller);
            let core_controller_input = CoreControllerInput::from_raw(&raw_controller_input);

            const INNER_DEAD_ZONE: f32 = 0.25;
            const OUTER_DEAD_ZONE: f32 = 0.75;
            let axis_transform = |v: Vec2| -> Vec2 {
                let mag = v.magnitude();
                let mag = if mag < INNER_DEAD_ZONE {
                    0.0
                } else if mag > OUTER_DEAD_ZONE {
                    1.0
                } else {
                    (mag - INNER_DEAD_ZONE) / (OUTER_DEAD_ZONE - INNER_DEAD_ZONE)
                };
                return v.normalized().scaled(mag);
            };
            GameControllerInput::from_core_and_transform(&core_controller_input, axis_transform)
        };
        println!("input: {:?}", input_state);

        // modify the game state based on the input
        input_system(&mut game_state, &input_state);

        // we want physics first because we want the world to be in a valid state before
        // running the entity logic.
        // Example: a homing missle goes through a wall
        physics_system(&mut game_state);
        game_state = entity_logic_system(game_state);

        render_system(&game_state);
        audio_system(&game_state);

        // wait for next frame
        let sleep_dur = std::time::Duration::from_millis(2000);
        std::thread::sleep(sleep_dur);
    }
    Ok(())
}

pub fn input_system(game_state: &mut GameState, input_state: &GameControllerInput) {
    game_state.player.velocity =
        Vec3::from_vec2(&input_state.ls).scaled(game_state.player.max_move_speed);
    game_state.player.shoot_dir = Vec3::from_vec2(&input_state.rs).normalized();
}
pub fn physics_system(game_state: &mut GameState) {
    let dt = std::time::Duration::from_millis(16);

    // update player
    game_state.player.position += game_state.player.velocity.scaled(dt.as_secs_f32());

    // update projectiles
    for p in &mut game_state.projectiles {
        if let Some(p) = p {
            p.position += p.velocity.scaled(dt.as_secs_f32());
        }
    }

    // update enemies
    for enemy in &mut game_state.enemies {
        if let Some(enemy) = enemy {
            enemy.position += enemy.velocity.scaled(dt.as_secs_f32());
        }
    }
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
