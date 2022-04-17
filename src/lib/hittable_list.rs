use crate::hittable::*;
use crate::Material;
use std::marker::PhantomData;

pub struct HittableList<M: Material, H: Hittable<M>> {
    objects: Vec<H>,
    phantom: PhantomData<M>,
}

impl<M: Material, H: Hittable<M>> HittableList<M, H> {
    pub fn new(obj: H) -> Self
    where
        H: Hittable<M>,
    {
        HittableList::<M, H> {
            objects: vec![obj],
            phantom: PhantomData,
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, obj: H)
    where
        H: Hittable<M>,
    {
        self.objects.push(obj)
    }
}

impl<M: Material, H: Hittable<M>> Hittable<M> for HittableList<M, H> {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<M>) -> bool
    where
        M: Material,
    {
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

impl<M: Material, H: Hittable<M>> Default for HittableList<M, H> {
    fn default() -> Self {
        Self {
            objects: vec![],
            phantom: PhantomData,
        }
    }
}
