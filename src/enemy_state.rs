use crate::math::{Point, Vector};

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum EnemyVariant {
    Basic = 0,
}
#[derive(Copy, Clone)]
pub struct EnemyState {
    pub position: Point,
    pub velocity: Vector,
    pub health: f32,
    pub variant: EnemyVariant,
}

impl EnemyState {
    pub fn new() -> EnemyState {
        EnemyState {
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
            health: 100.0,
            variant: EnemyVariant::Basic,
        }
    }
}
