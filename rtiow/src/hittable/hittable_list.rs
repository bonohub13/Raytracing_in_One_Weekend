use super::{Aabb, HitRecord, Hittable};
use crate::{interval::Interval, ray::Ray, vec3::Point3};
use std::sync::Arc;

#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::EMPTY,
        }
    }

    pub fn from(object: Arc<dyn Hittable>) -> Self {
        let mut ret = Self::new();

        ret.add(object);

        ret
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = Aabb::surrounding_box(
            &self.bbox,
            &object
                .bounding_box()
                .unwrap_or(Aabb::new(Point3::zeroes(), Point3::zeroes())),
        );
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox.clone())
    }
}
