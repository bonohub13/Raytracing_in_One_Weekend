use crate::{interval::Interval, PI};
use rand::prelude::*;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180_f64
}

#[inline]
pub fn random() -> f64 {
    thread_rng().gen()
}

#[inline]
pub fn random_in_range(range: &Interval) -> f64 {
    range.min + (range.max - range.min) * thread_rng().gen::<f64>()
}

#[inline]
pub fn random_i32(range: &Interval) -> i32 {
    random_in_range(&Interval::new(range.min, range.max + 1_f64)) as i32
}

#[test]
fn test_degrees_to_radians() {
    let degrees = 60_f64;
    let radians = degrees_to_radians(degrees);
    let target = PI / 3_f64;

    assert_eq!(target, radians)
}
