use crate::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f64,
}

impl Ray {
    #[inline]
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            orig: origin,
            dir: direction,
            time,
        }
    }

    #[inline]
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    #[inline]
    pub fn time(&self) -> f64 {
        self.time
    }

    #[inline]
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
