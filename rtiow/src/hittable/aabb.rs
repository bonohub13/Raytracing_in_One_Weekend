use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };
    pub const UNIVERSE: Self = Self {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    pub fn new(a: Point3, b: Point3) -> Self {
        let x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };
        let y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };
        let z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };

        let (x, y, z) = Self::pad_to_minimum(&x, &y, &z);

        Self { x, y, z }
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        Self {
            x: Interval::from(&box0.x, &box1.x),
            y: Interval::from(&box0.y, &box1.y),
            z: Interval::from(&box0.z, &box1.z),
        }
    }

    pub const fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1_f64 / ray_dir[axis];
            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;
            let (min, max) = if t0 < t1 {
                let min = if t0 > ray_t.min { t0 } else { ray_t.min };
                let max = if t1 < ray_t.max { t1 } else { ray_t.max };

                (min, max)
            } else {
                let min = if t1 > ray_t.min { t1 } else { ray_t.min };
                let max = if t0 < ray_t.max { t0 } else { ray_t.max };

                (min, max)
            };

            if max <= min {
                return false;
            }
        }

        true
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }

    fn pad_to_minimum(x: &Interval, y: &Interval, z: &Interval) -> (Interval, Interval, Interval) {
        const DELTA: f64 = 1e-4;

        let x = if x.size() < DELTA {
            x.expand(DELTA)
        } else {
            *x
        };
        let y = if y.size() < DELTA {
            y.expand(DELTA)
        } else {
            *y
        };
        let z = if z.size() < DELTA {
            z.expand(DELTA)
        } else {
            *z
        };

        (x, y, z)
    }
}

impl Add<Vec3> for Aabb {
    type Output = Self;

    fn add(self, offset: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + offset.x(),
            y: self.y + offset.y(),
            z: self.z + offset.z(),
        }
    }
}

impl Add<&Vec3> for Aabb {
    type Output = Self;

    fn add(self, offset: &Vec3) -> Self::Output {
        self + (*offset)
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, bbox: Aabb) -> Self::Output {
        bbox + self
    }
}

impl Add<&Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, bbox: &Aabb) -> Self::Output {
        bbox.clone() + self
    }
}
