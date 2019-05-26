#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }
    pub fn zero() -> Point {
        Point::new(0.0, 0.0, 0.0)
    }
}
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn from_vec2(v: &Vec2) -> Vec3 {
        Vec3 {
            x: v.x,
            y: v.y,
            z: 0.0,
        }
    }

    /// unit vector in the direction of the x-axis
    pub fn i_hat() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
    /// unit vector in the direction of the y-axis
    pub fn j_hat() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }
    /// unit vector in the direction of the z-axis
    pub fn k_hat() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }
    pub fn scale(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
    pub fn scaled(&self, s: f32) -> Vec3 {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
    pub fn normalize(&mut self) {
        let s = 1.0 / self.magnitude();
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
    pub fn normalized(&self) -> Vec3 {
        let s = 1.0 / self.magnitude();
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude2().sqrt()
    }

    pub fn magnitude2(&self) -> f32 {
        self.dot(self)
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
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
    extents: Vec3,
}
