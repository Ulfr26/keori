// Definitions of all the different data structures needed for rendering
// This defines vectors and vector arithmetic, (more to come)

use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone)]
pub struct Matrix {
    pub vals: [[f64; 4]; 4],
}

impl Vector {
    pub fn new() -> Vector {
        Vector {
            x: 0f64,
            y: 0f64,
            z: 0f64,
            w: 1f64,
        }
    }

    pub fn from(x: f64, y: f64, z: f64, w: f64) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn magnitude(self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).powf(0.5)
    }

    // Keep direction of vector but set magnitude to 1
    pub fn normalise(&mut self) {
        let m = self.clone().magnitude();

        self.x /= m;
        self.y /= m;
        self.z /= m;
    }

    pub fn normalised(self) -> Vector {
        let mut v = self.clone();

        v.normalise();
        v
    }

    pub fn dot_product(v1: &Vector, v2: &Vector) -> f64 {
        return v1.x*v2.x+v1.y*v2.y+v1.z*v2.z;
    }

    pub fn cross_product(v1: &Vector, v2: &Vector) -> Vector {
        return Vector::from(v1.y*v2.z-v1.z*v2.y, v1.z*v2.x-v1.x*v2.z, v1.x*v2.y-v1.y*v2.x, 0.0);
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector::from(self.x * rhs, self.y * rhs, self.z * rhs, self.w)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector::from(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector::from(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w)
    }
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            vals: [[0f64; 4]; 4],
        }
    }

    pub fn translation(v: Vector) -> Matrix {
        Matrix {
            vals: [[1.0, 0.0, 0.0, v.x],
                   [0.0, 1.0, 0.0, v.y],
                   [0.0, 0.0, 1.0, v.z],
                   [0.0, 0.0, 0.0, 1.0]]
        }
    }

    pub fn rotation(angle: f64, axis: Vector) -> Matrix {
        let uvec = axis.normalised();
        let ux = uvec.x;
        let uy = uvec.y;
        let uz = uvec.z;

        Matrix {
            vals: [
                [angle.cos()+ux.powf(2.0)*(1.0-angle.cos()), ux*uy*(1.0-angle.cos())-uz*angle.sin(), ux*uz*(1.0-angle.cos())+uy*angle.sin(), 0.0],
                [uy*ux*(1.0-angle.cos())+uz*angle.sin(), angle.cos()+uy.powf(2.0)*(1.0-angle.cos()), uy*uz*(1.0-angle.cos())-ux*angle.sin(), 0.0],
                [uz*ux*(1.0-angle.cos())-uy*angle.sin(), uz*uy*(1.0-angle.cos())+ux*angle.sin(), angle.cos()+uz.powf(2.0)*(1.0-angle.cos()), 0.0],
                [0.0, 0.0, 0.0, 0.0]
            ]
        }
    }

    pub fn perspective(angle: f64, ratio: f64, near: f64, far: f64) -> Matrix {
        // (THA stands for tan-half-angle - thought the variable name was unwieldy
        let tha = (angle/2f64).tan();

        Matrix {
            vals: [[1.0/(ratio * tha), 0.0, 0.0, 0.0],
                   [0.0, 1.0/tha, 0.0, 0.0],
                   [0.0, 0.0, -(far + near)/(far - near), -(2.0*far*near)/(far-near)],
                   [0.0, 0.0, -1.0, 0.0]]
        }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self.vals[0][0]*rhs.x + self.vals[0][1]*rhs.y + self.vals[0][2]*rhs.z + self.vals[0][3]*rhs.w,
            y: self.vals[1][0]*rhs.x + self.vals[1][1]*rhs.y + self.vals[1][2]*rhs.z + self.vals[1][3]*rhs.w,
            z: self.vals[2][0]*rhs.x + self.vals[2][1]*rhs.y + self.vals[2][2]*rhs.z + self.vals[2][3]*rhs.w,
            w: self.vals[3][0]*rhs.x + self.vals[3][1]*rhs.y + self.vals[3][2]*rhs.z + self.vals[3][3]*rhs.w,
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut vals = [[0f64; 4]; 4];
        
        // Watch as I condense matrix multiplication into the worst thing ever:
        // A triple nested for loop. Fun for the whole family! That is if you have a family of
        // masochists, though frankly I think 'masochist' and 'programmer' mean the same thing.
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    vals[i][j] += self.vals[i][k]*rhs.vals[k][j];
                }
            }
        }

        // Bruh
        Matrix {
            vals: vals,
        }
    }
}
