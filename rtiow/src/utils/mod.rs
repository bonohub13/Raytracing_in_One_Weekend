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
    thread_rng().gen_range(range.min..=range.max)
}

#[test]
fn test_degrees_to_radians() {
    let degrees = 60_f64;
    let radians = degrees_to_radians(degrees);
    let target = PI / 3_f64;

    assert_eq!(target, radians)
}