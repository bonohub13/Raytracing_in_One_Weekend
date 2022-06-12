use crate::Hittable;
use crate::{Aabb, HitRecord, Ray, Vec3};

pub struct Translate<H: Hittable> {
    pub hittable: H,
    offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(offset: Vec3, hittable: H) -> Self {
        Self { hittable, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if let Some(mut rec) = self.hittable.hit(&moved_r, t_min, t_max) {
            rec.set_p(rec.p() + self.offset);
            rec.set_face_normal(&moved_r, &rec.normal());

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if let Some(bounding_box) = self.hittable.bounding_box(time0, time1) {
            Some(Aabb::new(
                bounding_box.min() + self.offset,
                bounding_box.max() + self.offset,
            ))
        } else {
            None
        }
    }
}
