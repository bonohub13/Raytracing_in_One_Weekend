use crate::PI;
use crate::{dot, surrounding_box};
use crate::{Aabb, HitRecord, Point3, Ray, Vec3};
use crate::{Hittable, Material};

pub struct MovingSphere<M: Material> {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: M,
}

impl<M: Material> MovingSphere<M> {
    #[inline]
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: M,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    #[inline]
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant >= 0. {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;

            if root >= t_min && t_max >= root {
                let p = r.at(root);
                let normal = (p - self.center(r.time())) / self.radius;
                let theta = -p.y().acos();
                let phi = -p.z().atan2(p.x()) + PI;
                let mut rec =
                    HitRecord::new(p, normal, root, phi / (2. * PI), theta / PI, &self.material);

                rec.set_face_normal(r, &normal);

                return Some(rec);
            }

            root = (-half_b + sqrtd) / a;

            if root >= t_min && t_max >= root {
                let p = r.at(root);
                let normal = (p - self.center(r.time())) / self.radius;
                let theta = -p.y().acos();
                let phi = -p.z().atan2(p.x()) + PI;
                let mut rec =
                    HitRecord::new(p, normal, root, phi / (2. * PI), theta / PI, &self.material);

                rec.set_face_normal(r, &normal);

                return Some(rec);
            }
        }

        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(surrounding_box(box0, box1))
    }
}
