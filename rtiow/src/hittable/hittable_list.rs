use super::{HitRecord, Hittable};
use crate::{interval::Interval, ray::Ray};
use std::sync::Arc;

#[derive(Debug)]
pub struct HittableList<T: Hittable> {
    objects: Vec<Arc<T>>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn from(object: Arc<T>) -> Self {
        let mut ret = Self::new();

        ret.add(object);

        ret
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Arc<T>) {
        self.objects.push(object)
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
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
}
