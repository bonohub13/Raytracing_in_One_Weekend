use crate::{
    interval::Interval,
    utils::{random, random_in_range},
};
use std::ops::{AddAssign, DivAssign, Index, MulAssign, Neg};

mod color;
mod utils;

#[cfg(test)]
mod test_utils;

pub use color::*;
pub use utils::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub use Vec3 as Color;
pub use Vec3 as Point3;

impl Vec3 {
    const DELTA_ZERO: f64 = 1e-8;

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn zeroes() -> Self {
        Self { e: [0_f64; 3] }
    }

    pub const fn size(&self) -> usize {
        self.e.len()
    }

    pub const fn x(&self) -> f64 {
        self.e[0]
    }

    pub const fn y(&self) -> f64 {
        self.e[1]
    }

    pub const fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        let v = *self;

        (v * v).e.iter().sum()
    }

    pub fn near_zero(&self) -> bool {
        (self.e[0].abs() < Self::DELTA_ZERO)
            && (self.e[1].abs() < Self::DELTA_ZERO)
            && (self.e[2].abs() < Self::DELTA_ZERO)
    }

    pub fn random() -> Self {
        Self {
            e: [random(), random(), random()],
        }
    }

    pub fn random_in_range(range: &Interval) -> Self {
        Self {
            e: [
                random_in_range(range),
                random_in_range(range),
                random_in_range(range),
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        for (i, e_i) in self.e.iter_mut().enumerate() {
            *e_i += other[i];
        }
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Self) {
        for (i, e_i) in self.e.iter_mut().enumerate() {
            *e_i += other[i];
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        for e_i in self.e.iter_mut() {
            *e_i *= t;
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1_f64 / t;
    }
}

#[test]
fn test_length_squared() {
    let v = Vec3::new(2_f64, 3_f64, 5_f64);
    let target = 38_f64; // 4 + 9 + 25

    assert_eq!(target, v.length_squared())
}

#[test]
fn test_length() {
    let v = Vec3::new(3_f64, 4_f64, 5_f64);
    let target = 50_f64.sqrt();

    assert_eq!(target, v.length());
}

#[test]
fn test_neg() {
    let v = Vec3::new(1_f64, -1_f64, 0.1);
    let target = Vec3::new(-1_f64, 1_f64, -0.1);

    assert_eq!(target, -v)
}

#[test]
fn test_index() {
    let v = Vec3::new(2_f64, 3_f64, 5_f64);

    assert_eq!(2_f64, v[0]);
    assert_eq!(3_f64, v[1]);
    assert_eq!(5_f64, v[2]);
}

#[test]
fn test_add_assign() {
    let mut v = Vec3::new(1_f64, -1_f64, 0.1);
    let u = Vec3::new(2_f64, 10_f64, 0.01);
    let target = Vec3::new(3_f64, 9_f64, 0.11);

    v += u;

    assert_eq!(target, v)
}

#[test]
fn test_add_assign_ref() {
    let mut v = Vec3::new(1_f64, -1_f64, 0.1);
    let u = Vec3::new(2_f64, 10_f64, 0.01);
    let target = Vec3::new(3_f64, 9_f64, 0.11);

    v += &u;

    assert_eq!(target, v)
}

#[test]
fn test_mul_assign() {
    let mut v = Vec3::new(1_f64, -1_f64, 0.1);
    let t = 0.1;
    let target = Vec3::new(0.1, -0.1, 0.01);

    v *= t;
    v.e.iter_mut()
        .for_each(|e_i| *e_i = (1e3 * *e_i).round() / 1e3);

    assert_eq!(target, v)
}

#[test]
fn test_div_assign() {
    let mut v = Vec3::new(1_f64, -1_f64, 0.1);
    let t = 0.5;
    let target = Vec3::new(2_f64, -2_f64, 0.2);

    v /= t;

    assert_eq!(target, v)
}

#[test]
fn test_near_zero() {
    let v = Vec3 {
        e: [999e-11, 99e-10, 0_f64],
    };
    let u = Vec3 {
        e: [1e-8, 99e-10, 101e-10],
    };

    assert!(v.near_zero());
    assert!(!u.near_zero());
}
