use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let org = Point3::new(0.0, 0.0, 0.0);
        let hor = Vec3::new(viewport_width, 0.0, 0.0);
        let ver = Vec3::new(0.0, viewport_height, 0.0);

        Self {
            origin: org,
            horizontal: hor,
            vertical: ver,
            lower_left_corner: org - hor / 2 - ver / 2 - Vec3::new(0.0, 0.0, focal_length),
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
