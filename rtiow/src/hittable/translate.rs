use super::{Aabb, HitRecord, Hittable};
use crate::{
    interval::Interval,
    ray::Ray,
    utils,
    vec3::{Point3, Vec3},
    INFINITY,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
}

#[derive(Debug)]
pub struct RotateY {
    object: Arc<dyn Hittable>,
    theta: [f64; 2],
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: &Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self {
            object: object.clone(),
            offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let offset_r = Ray::new(*r.origin() - self.offset, *r.direction(), *r.time());

        if let Some(mut rec) = self.object.hit(&offset_r, ray_t) {
            rec.p += self.offset;

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(
            self.object
                .bounding_box()
                .unwrap_or(Aabb::new(Vec3::zeroes(), Vec3::zeroes()))
                + self.offset,
        )
    }
}

impl RotateY {
    pub fn new(object: &Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = utils::degrees_to_radians(angle);
        let theta = [radians.cos(), radians.sin()];
        let bbox = object
            .bounding_box()
            .unwrap_or(Aabb::new(Vec3::zeroes(), Vec3::zeroes()));
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        (0..2).into_iter().for_each(|i| {
            (0..2).into_iter().for_each(|j| {
                (0..2).into_iter().for_each(|k| {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;
                    let newx = theta[0] * x + theta[1] * z;
                    let newz = -theta[1] * x + theta[0] * z;
                    let tester = Vec3::new(newx, y, newz);

                    (0..3).into_iter().for_each(|c| {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    });
                })
            });
        });

        Self {
            object: object.clone(),
            theta,
            bbox: Aabb::new(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let origin = Point3::new(
            (self.theta[0] * r.origin().x()) - (self.theta[1] * r.origin().z()),
            r.origin().y(),
            (self.theta[1] * r.origin().x()) + (self.theta[0] * r.origin().z()),
        );
        let direction = Vec3::new(
            (self.theta[0] * r.direction().x()) - (self.theta[1] * r.direction().z()),
            r.direction().y(),
            (self.theta[1] * r.direction().x()) + (self.theta[0] * r.direction().z()),
        );
        let rotated_y = Ray::new(origin, direction, *r.time());

        if let Some(mut rec) = self.object.hit(&rotated_y, ray_t) {
            rec.p = Point3::new(
                (self.theta[0] * rec.p.x()) - (self.theta[1] * rec.p.z()),
                rec.p.y(),
                (-self.theta[1] * rec.p.x()) + (self.theta[0] * rec.p.z()),
            );
            rec.normal = Vec3::new(
                (self.theta[0] * rec.normal.x()) - (self.theta[1] * rec.normal.z()),
                rec.normal.y(),
                (-self.theta[1] * rec.normal.x()) + (self.theta[0] * rec.normal.z()),
            );

            Some(rec)
        } else {
            None
        }
    }
}
