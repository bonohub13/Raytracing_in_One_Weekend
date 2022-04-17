use crate::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    // Creates a new instance Ray and returns it
    pub fn new(origin: crate::Point3, direction: crate::Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    // Methods
    pub fn at(&self, rhs: f64) -> crate::Point3 {
        self.orig + rhs * self.dir
    }
    // Getters
    pub fn origin(&self) -> crate::Point3 {
        self.orig
    }
    pub fn direction(&self) -> crate::Vec3 {
        self.dir
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            orig: Point3::default(),
            dir: Vec3::default(),
        }
    }
}
