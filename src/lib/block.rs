use crate::{Aabb, HitRecord, HittableList, Point3, Ray, RectXY, RectXZ, RectYZ};
use crate::{Hittable, Material};

pub struct Box {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl super::Box {
    pub fn new(p0: Point3, p1: Point3, material: impl Material + 'static + Copy) -> Self {
        let mut sides = HittableList::default();

        sides.push(RectXY::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            material,
        ));
        sides.push(RectXY::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            material,
        ));
        sides.push(RectXZ::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            material,
        ));
        sides.push(RectXZ::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            material,
        ));
        sides.push(RectYZ::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            material,
        ));
        sides.push(RectYZ::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            material,
        ));

        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hittable for Box {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}
