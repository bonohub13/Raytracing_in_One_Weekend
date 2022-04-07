use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

// Operator overloading
// Negative
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

// Indexing
impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

// Addition
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}
impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}
impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

// Vec * f64
impl ops::MulAssign<i32> for Vec3 {
    fn mul_assign(&mut self, rhs: i32) {
        *self = Self {
            e: [
                self.e[0] * rhs as f64,
                self.e[1] * rhs as f64,
                self.e[2] * rhs as f64,
            ],
        }
    }
}
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}
impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}
impl ops::Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs as f64,
                self.e[1] * rhs as f64,
                self.e[2] * rhs as f64,
            ],
        }
    }
}
impl ops::Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self as f64 * rhs.e[0],
                self as f64 * rhs.e[1],
                self as f64 * rhs.e[2],
            ],
        }
    }
}
impl ops::DivAssign<i32> for Vec3 {
    fn div_assign(&mut self, rhs: i32) {
        *self *= 1.0 / rhs as f64
    }
}
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}
impl ops::Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        self * (1.0 / rhs as f64)
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

// Print formatting
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }
    // Getters
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    // Methods
    // Returns length of 3D Vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    // Returns squared size of 3D Vector
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

pub fn dot(v: &Vec3, rhs: &Vec3) -> f64 {
    v.e[0] * rhs.e[0] + v.e[1] * rhs.e[1] + v.e[2] * rhs.e[2]
}

pub fn cross(v: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            v.e[1] * rhs.e[2] - v.e[2] * rhs.e[1],
            v.e[2] * rhs.e[0] - v.e[0] * rhs.e[2],
            v.e[0] * rhs.e[1] - v.e[1] * rhs.e[0],
        ],
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

// Aliases
pub use self::Vec3 as Point3;
pub use self::Vec3 as Color;
