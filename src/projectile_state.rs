use crate::math::{Point, Vector};

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ProjectileVariant {
    Basic = 0,
}

#[derive(Copy, Clone)]
pub struct ProjectileState {
    pub position: Point,
    pub velocity: Vector,
    pub variant: ProjectileVariant,
}

impl ProjectileState {
    fn new() -> ProjectileState {
        ProjectileState {
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
            variant: ProjectileVariant::Basic,
        }
    }
}
