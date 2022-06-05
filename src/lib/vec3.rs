use crate::{random_f64, random_f64_in_range};
use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        let mut length_squared = 0.0;

        for i in self.e.iter() {
            length_squared += i * i;
        }

        length_squared
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn random() -> Self {
        Self::new(random_f64(), random_f64(), random_f64())
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        Self::new(
            random_f64_in_range(min, max),
            random_f64_in_range(min, max),
            random_f64_in_range(min, max),
        )
    }

    pub fn is_near_zero(&self) -> bool {
        const S: f64 = 1e-8;

        (self[0].abs() < S) && (self[1].abs() < S) && (self[2].abs() < S)
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    let mut dot = 0.0;

    for e in (*v * *u).e.iter() {
        dot += e;
    }

    dot
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    let mut cross = Vec3::default();

    for i in 0..3 {
        cross[i] = u.e[(i + 1) % 3] * v.e[(i + 2) % 3] - u.e[(i + 2) % 3] * v.e[(i + 1) % 3]
    }

    cross
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_in_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();

    if dot(&in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v, n) * *n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let mut cos_theta = dot(&-*uv, n);

    if cos_theta > 1.0 {
        cos_theta = 1.0;
    }

    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;

    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_f64_in_range(-1., 1.),
            random_f64_in_range(-1., 1.),
            0.,
        );

        if p.length_squared() < 1. {
            return p;
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

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

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

// Type aliases
pub use self::Vec3 as Point3;
pub use self::Vec3 as Color;
