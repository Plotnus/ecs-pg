#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    pub fn normalize(&mut self) {
        let s = 1.0 / self.magnitude();
        self.x *= s;
        self.y *= s;
    }
    pub fn normalized(&self) -> Vec2 {
        let s = 1.0 / self.magnitude();
        Vec2 {
            x: self.x * s,
            y: self.y * s,
        }
    }

    pub fn scale(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
    }
    pub fn scaled(&self, s: f32) -> Vec2 {
        Vec2 {
            x: self.x * s,
            y: self.y * s,
        }
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude2().sqrt()
    }

    pub fn magnitude2(&self) -> f32 {
        self.dot(&self)
    }

    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}
#[derive(Copy, Clone)]
pub struct AABB {
    center: Point,
    extents: Vector,
}
