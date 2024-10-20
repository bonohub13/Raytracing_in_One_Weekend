use super::{Aabb, HitRecord, Hittable, HittableList};
use crate::{interval::Interval, ray::Ray, vec3::Point3};
use anyhow::Result;
use rayon::prelude::*;
use std::{cmp::Ordering, sync::Arc};

#[derive(Debug)]
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Result<Self> {
        let mut bbox = Aabb::EMPTY;

        for object_index in start..end {
            bbox = Aabb::surrounding_box(
                &bbox,
                &objects[object_index]
                    .bounding_box()
                    .expect("Bounding box not available"),
            );
        }

        let axis = bbox.longest_axis();
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };
        let object_span = end - start;

        let (left, right) = match object_span {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => (objects[start].clone(), objects[start + 1].clone()),
            _ => {
                objects[start..end].par_sort_by(|a, b| comparator(a, b));

                let mid = start + object_span / 2;
                let left: Arc<dyn Hittable> = Arc::new(Self::new(objects, start, mid)?);
                let right: Arc<dyn Hittable> = Arc::new(Self::new(objects, mid, end)?);

                (left, right)
            }
        };

        Ok(Self { left, right, bbox })
    }

    pub fn from(list: &HittableList) -> Result<Self> {
        let mut objects = list.objects.clone();
        let length = objects.len();

        Self::new(&mut objects, 0, length)
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_axis_interval = a
            .bounding_box()
            .unwrap_or(Aabb::new(Point3::zeroes(), Point3::zeroes()));
        let b_axis_interval = b
            .bounding_box()
            .unwrap_or(Aabb::new(Point3::zeroes(), Point3::zeroes()));

        a_axis_interval
            .axis_interval(axis_index)
            .min
            .partial_cmp(&b_axis_interval.axis_interval(axis_index).min)
            .expect("No bounding box in Bvh::new")
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_left = if let Some(rec) = self.left.hit(r, ray_t) {
            Some(rec)
        } else {
            None
        };
        let t = hit_left.as_ref().map(|rec| rec.t).unwrap_or(ray_t.max);
        let hit_right = if let Some(rec) = self.right.hit(r, &Interval::new(ray_t.min, t)) {
            Some(rec)
        } else {
            None
        };

        hit_left.or_else(|| hit_right)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox.clone())
    }
}
