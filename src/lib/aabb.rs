use crate::{max, min};
use crate::{Point3, Ray};
use std::mem;

// Axis-Aligned Bounding Boxes
#[derive(Clone, Copy, PartialEq)]
pub struct Aabb {
    minimum: Point3,
    maximum: Point3,
}

impl Aabb {
    #[inline]
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }

    #[inline]
    pub fn min(&self) -> Point3 {
        self.minimum
    }

    #[inline]
    pub fn max(&self) -> Point3 {
        self.maximum
    }

    #[inline]
    pub fn hit(
        &self,
        r: &Ray,
        mut t_min: f64,
        mut t_max: f64,
    ) -> Option<(Option<f64>, Option<f64>)> {
        for a in 0..3 {
            let inv_d = 1. / r.direction()[a];
            let mut t0 = min(
                (self.min()[a] - r.origin()[a]) * inv_d,
                (self.max()[a] - r.origin()[a]) * inv_d,
            );
            let mut t1 = max(
                (self.minimum[a] - r.origin()[a]) / r.direction()[a],
                (self.maximum[a] - r.origin()[a]) / r.direction()[a],
            );

            if inv_d < 0. {
                mem::swap(&mut t0, &mut t1);
            }

            t_min = max(t0, t_min);
            t_max = min(t1, t_max);

            if t_max <= t_min {
                return Some((None, None));
            }
        }

        Some((Some(t_min), Some(t_max)))
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = Point3::new(
        min(box0.min().x(), box1.min().x()),
        min(box0.min().y(), box1.min().y()),
        min(box0.min().z(), box1.min().z()),
    );
    let big = Point3::new(
        max(box0.min().x(), box1.min().x()),
        max(box0.min().y(), box1.min().y()),
        max(box0.min().z(), box1.min().z()),
    );

    Aabb::new(small, big)
}
