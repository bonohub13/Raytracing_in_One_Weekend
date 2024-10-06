use super::Vec3;
use crate::interval::Interval;
use std::{
    iter::{Iterator, Sum},
    ops::{Add, Div, Mul, Sub},
};

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut ret = self;

        ret += other;

        ret
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        let mut ret = self;

        ret += other;

        ret
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        self + -(*other)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut e = [0_f64; 3];

        e.iter_mut().enumerate().for_each(|(i, e_i)| {
            *e_i = self[i] * other[i];
        });

        Self { e }
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: &Self) -> Self::Output {
        self * (*other)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        let mut ret = self;

        ret *= t;

        ret
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        *other * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        (1_f64 / t) * self
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zeroes(), |a, b| a + b)
    }
}

impl<'a> Sum<&'a Vec3> for Vec3 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zeroes(), |a, b| a + b)
    }
}

#[inline]
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e.iter().enumerate().map(|(i, e_i)| e_i * v[i]).sum()
}

#[inline]
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    let mut e = [0_f64; 3];

    e.iter_mut().enumerate().for_each(|(i, e_i)| {
        *e_i = u[(i + 1) % 3] * v[(i + 2) % 3] - u[(i + 2) % 3] * v[(i + 1) % 3]
    });

    Vec3 { e }
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_in_range(&Interval::new(-1_f64, 1_f64));
        let lensq = p.length_squared();

        if 1e-160 < lensq && lensq <= 1_f64 {
            return p / lensq.sqrt();
        }
    }
}

#[inline]
pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();

    if dot(&on_unit_sphere, normal) > 0_f64 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

#[inline]
pub fn reflect(v: &Vec3, u: &Vec3) -> Vec3 {
    *v - 2_f64 * dot(v, u) * u
}

#[inline]
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&(-(*uv)), n).min(1_f64);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * n);
    let r_out_parallel = -(1_f64 - r_out_perp.length_squared()).abs().sqrt() * n;

    r_out_perp + r_out_parallel
}

#[test]
fn test_dot() {
    let v = Vec3::new(2_f64, 3_f64, 5_f64);
    let u = Vec3::new(7_f64, 11_f64, 13_f64);
    let target = 112_f64; // 14 + 33 + 65

    assert_eq!(target, dot(&v, &u));
}

#[test]
fn test_cross() {
    let v = Vec3::new(2_f64, 3_f64, 5_f64);
    let u = Vec3::new(7_f64, 11_f64, 13_f64);
    let target = Vec3::new(
        -16_f64, // 39 - 55
        9_f64,   // 35 - 26
        1_f64,   // 22 - 21
    );

    assert_eq!(target, cross(&v, &u));
}

#[test]
fn test_unit_vector() {
    let v = Vec3::new(2_f64, 3_f64, 5_f64);
    let target = Vec3::new(
        2_f64 / 38_f64.sqrt(), // 2_f64 / (4_f64 + 9_f64 + 25_f64).sqrt()
        3_f64 / 38_f64.sqrt(), // 3_f64 / (4_f64 + 9_f64 + 25_f64).sqrt()
        5_f64 / 38_f64.sqrt(), // 5_f64 / (4_f64 + 9_f64 + 25_f64).sqrt()
    );

    assert_eq!(target, unit_vector(&v));
}

#[test]
fn test_reflect() {
    let v = Vec3::new(2_f64, 3_f64, 5_f64);
    let u = Vec3::new(7_f64, 11_f64, 13_f64);
    let result = reflect(&v, &u);
    let target = Vec3::new(
        -1566_f64, // 2_f64 - 2_f64 * (112_f64) * 7_f64
        -2461_f64, // 3_f64 - 2_f64 * (112_f64) * 11_f64
        -2907_f64, // 5_f64 - 2_f64 * (112_f64) * 13_f64
    );

    assert_eq!(target, result);
}

#[test]
fn test_refract() {
    let uv = Vec3::new(2_f64, -3_f64, 5_f64);
    let n = Vec3::new(-7_f64, 11_f64, -13_f64);
    let etai_over_etat = 17_f64;
    let result = refract(&uv, &n, etai_over_etat);
    let scala = -44216_f64.sqrt();
    // cos_theta: 1_f64
    // r_out_perp: [17_f64 * 5_f64, 17_f64 * -8_f64, 17_f64, -8_f64]
    // r_out_prallel: [scala * -7_f64, scala * 11_f64, scala * -13_f64]
    let target = Vec3::new(
        -85_f64 + scala * -7_f64,   // -85_f64 + scala * -7_f64
        136_f64 + scala * 11_f64,   // 136_f64 + scala * 11_f64
        -136_f64 + scala * -13_f64, // -136_f64 + scala * -13_f64
    );

    assert_eq!(target, result);
}
