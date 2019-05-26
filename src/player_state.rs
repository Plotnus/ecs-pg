use crate::math::{Point, Vec3};
pub struct PlayerState {
    pub position: Point,
    pub velocity: Vec3,
    pub shoot_dir: Vec3,
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
            velocity: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            shoot_dir: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max_move_speed: 5.0,
        }
    }
}
