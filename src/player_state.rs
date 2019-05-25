use crate::math::{Point, Vec3};
pub struct PlayerState {
    pub position: Point,
    pub move_dir: Vec3,
    pub max_move_speed: f32,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            position: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            move_dir: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max_move_speed: 0.0,
        }
    }
}
