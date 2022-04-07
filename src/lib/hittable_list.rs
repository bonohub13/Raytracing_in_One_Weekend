use crate::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(obj: Box<dyn Hittable>) -> Self {
        HittableList { objects: vec![obj] }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec = HitRecord::default();

        for obj in self.objects.iter() {
            if obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                rec.set_p(&temp_rec.p());
                rec.set_normal(&temp_rec.normal());
                rec.set_t(temp_rec.t());
                rec.set_front_face(&temp_rec.front_face());
            }
        }

        hit_anything
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self { objects: vec![] }
    }
}
