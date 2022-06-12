use crate::degrees_to_radians;
use crate::Hittable;
use crate::INFINITY;
use crate::{Aabb, HitRecord, Point3, Ray, Vec3};

pub struct RotateY<H: Hittable> {
    pub hittable: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(hittable: H, angle: f64) -> Self {
        let randians = degrees_to_radians(angle);
        let sin_theta = randians.sin();
        let cos_theta = randians.cos();

        if let Some(bbox) = hittable.bounding_box(0., 1.) {
            let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
            let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                        let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                        let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();
                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;
                        let tester = Vec3::new(newx, y, newz);

                        for c in 0..3 {
                            min[c] = if min[c] < tester[c] {
                                min[c]
                            } else {
                                tester[c]
                            };
                            max[c] = if max[c] < tester[c] {
                                tester[c]
                            } else {
                                max[c]
                            };
                        }
                    }
                }
            }
            Self {
                hittable,
                sin_theta,
                cos_theta,
                bbox: Some(Aabb::new(min, max)),
            }
        } else {
            Self {
                hittable,
                sin_theta,
                cos_theta,
                bbox: None,
            }
        }
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];
        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(mut hit) = self.hittable.hit(&rotated_r, t_min, t_max) {
            let mut p = hit.p();
            let mut normal = hit.normal();

            p[0] = self.cos_theta * hit.p()[0] + self.sin_theta * hit.p()[2];
            p[2] = -self.sin_theta * hit.p()[0] + self.cos_theta * hit.p()[2];
            normal[0] = self.cos_theta * hit.normal()[0] + self.sin_theta * hit.normal()[2];
            normal[2] = -self.sin_theta * hit.normal()[0] + self.cos_theta * hit.normal()[2];

            hit.set_p(p);
            hit.set_face_normal(&rotated_r, &normal);

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}
