// Definitions of all the different data structures needed for rendering
// This defines vectors and vector arithmetic, (more to come)

use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new() -> Vector3 {
        Vector3 {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }

    pub fn from(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    // Cross product between two vectors
    pub fn cross(v1: Vector3, v2: Vector3) -> Vector3 {
        Vector3 {
            x: v1.y*v2.z - v1.z*v2.y,
            y: v1.z*v2.x - v1.x*v2.z,
            z: v1.x*v2.y - v1.y*v2.x,
        }
    }

    // Keep direction of vector but set magnitude to 1
    pub fn normalise(&mut self) {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let m = (x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).powf(2.0);
        self.x = x/m;
        self.y = y/m;
        self.z = z/m;
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}
