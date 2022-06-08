use crate::surrounding_box;
use crate::Hittable;
use crate::{Aabb, HitRecord, Ray};

#[derive(Default)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    #[inline(always)]
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.list.push(Box::new(hittable))
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;

        for h in self.list.iter() {
            if let Some(hit) = h.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t();
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let mut first_box = true;
        let mut output_box: Option<Aabb> = None;

        if !self.list.is_empty() {
            for h in self.list.iter() {
                if let Some(bounding_box) = h.bounding_box(time0, time1) {
                    output_box = if first_box {
                        Some(bounding_box)
                    } else {
                        Some(surrounding_box(output_box.unwrap(), bounding_box))
                    };
                    first_box = false;
                }
            }

            output_box
        } else {
            None
        }
    }
}
