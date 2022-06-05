use crate::{HitRecord, Hittable, Ray};

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
}
