use crate::math::Vec2;
pub struct InputState {
    // thumb-sticks
    ls: Vec2,
    rs: Vec2,
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
