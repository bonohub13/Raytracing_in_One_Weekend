use crate::{interval::Interval, PI};
use rand::prelude::*;

static mut RAND_INSTANCE: Option<ThreadRng> = None;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random() -> f64 {
    match unsafe { RAND_INSTANCE.clone() } {
        Some(mut thread) => thread.gen(),
        None => {
            unsafe { RAND_INSTANCE = Some(thread_rng()) };

            random()
        }
    }
}

#[inline]
pub fn random_in_range(range: &Interval) -> f64 {
    match unsafe { RAND_INSTANCE.clone() } {
        Some(mut thread) => thread.gen_range(range.min..=range.max),
        None => {
            unsafe { RAND_INSTANCE = Some(thread_rng()) };

            random_in_range(range)
        }
    }
}

#[test]
fn test_degrees_to_radians() {
    let degrees = 60.0;
    let radians = degrees_to_radians(degrees);
    let target = PI / 3.0;

    assert_eq!(target, radians)
}
