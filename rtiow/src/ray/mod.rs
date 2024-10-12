use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub const fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub const fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

#[test]
fn test_ray_at() {
    let origin = Point3::new(0_f64, 1_f64, 2_f64);
    let direction = Vec3::new(3_f64, 5_f64, 7_f64);
    let ray = Ray::new(origin, direction);
    let t = 0.5;
    let result = ray.at(t);
    let target = Point3::new(1.5, 3.5, 5.5);

    assert_eq!(target, result)
}