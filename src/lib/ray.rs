#[derive(Clone, Copy)]
pub struct Ray {
    orig: crate::Point3,
    dir: crate::Vec3,
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
