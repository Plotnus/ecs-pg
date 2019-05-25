use crate::math::Vec2;
#[derive(Debug)]
pub struct InputState {
    // thumb-sticks
    pub ls: Vec2,
    pub rs: Vec2,
    /* ... etc ... */
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            ls: Vec2 { x: 0.0, y: 0.0 },
            rs: Vec2 { x: 0.0, y: 0.0 },
        }
    }
}
#[derive(Debug)]
pub struct RawControllerInput {
    pub ls_x: i16,
    pub ls_y: i16,
    pub rs_x: i16,
    pub rs_y: i16,
}
/// this is just a copy of the input state for the controller
/// it does not do float representations or any reformatting
// one concern is what happens when a button goes down and up on one frame
// for our purposes we won't have to address that
impl RawControllerInput {
    pub fn new() -> RawControllerInput {
        RawControllerInput {
            ls_x: 0,
            ls_y: 0,
            rs_x: 0,
            rs_y: 0,
        }
    }
    pub fn from_sdl2_controller(
        controller: &sdl2::controller::GameController,
    ) -> RawControllerInput {
        RawControllerInput {
            ls_x: controller.axis(sdl2::controller::Axis::LeftX),
            ls_y: controller.axis(sdl2::controller::Axis::LeftY),
            rs_x: controller.axis(sdl2::controller::Axis::RightX),
            rs_y: controller.axis(sdl2::controller::Axis::RightY),
        }
    }
}
/// this is created from a RawControllerInputState.
pub struct CoreControllerInput {
    pub ls: Vec2,
    pub rs: Vec2,
}

impl CoreControllerInput {
    pub fn from_raw(raw: &RawControllerInput) -> CoreControllerInput {
        // range from [-1,1] with y-axis flipped so up is (+) and down is (-)
        let ls = Vec2::new(normalize_i16(raw.ls_x), -normalize_i16(raw.ls_y));
        let rs = Vec2::new(normalize_i16(raw.rs_x), -normalize_i16(raw.rs_y));

        // clamp magnitude between [0,1]
        let ls_mag = ls.magnitude().clamp(0.0, 1.0);
        let rs_mag = rs.magnitude().clamp(0.0, 1.0);

        // create so input is between 0 and 1
        CoreControllerInput {
            ls: ls.normalized().scaled(ls_mag),
            rs: rs.normalized().scaled(rs_mag),
        }
    }
}
fn normalize_i16(x: i16) -> f32 {
    if x < 0 {
        x as f32 / std::i16::MIN as f32
    } else {
        x as f32 / std::i16::MAX as f32
    }
}

///
#[derive(Debug)]
pub struct GameControllerInput {
    pub ls: Vec2,
    pub rs: Vec2,
}
impl GameControllerInput {
    /*
    pub fn new() -> GameControllerInput {
        GameControllerInput {
            ls: Vec2::new(0.0, 0.0),
            rs: Vec2::new(0.0, 0.0),
        }
    }
    */
    pub fn from_core_and_transform<F>(
        core_controller_input: &CoreControllerInput,
        stick_transform: F,
    ) -> GameControllerInput
    where
        F: Fn(Vec2) -> Vec2,
    {
        GameControllerInput {
            ls: stick_transform(core_controller_input.ls),
            rs: stick_transform(core_controller_input.rs),
        }
    }
}
