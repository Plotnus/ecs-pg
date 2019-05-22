use crate::math::{Point, Vector};
pub struct PlayerState {
    pub position: Point,
    pub velocity: Vector,
    pub health: f32,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            position: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            health: 0.0,
        }
    }
}
