use crate::surrounding_box;
use crate::Hittable;
use crate::{Aabb, HitRecord, Ray};
use anyhow::format_err;
use rand::Rng;
use rayon::slice::ParallelSliceMut;
use std::cmp::Ordering;

const MAX_SEQUENTIAL: usize = 250;

enum Node {
    Branch { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hittable>),
}

pub struct Bvh {
    tree: Node,
    bounding_box: Aabb,
}

impl Bvh {
    pub fn new(mut world: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> anyhow::Result<Self> {
        #[inline]
        fn box_compare(axis: usize) -> impl Fn(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            move |left, right| match (left.bounding_box(0., 0.), right.bounding_box(0., 0.)) {
                (Some(left), Some(right)) => {
                    left.min()[axis].partial_cmp(&right.min()[axis]).unwrap()
                }
                _ => panic!("No bounding box in Bvh::new() constructor."),
            }
        }

        let axis: usize = rand::thread_rng().gen_range(0..3);
        world.par_sort_unstable_by(box_compare(axis));

        match world.len() {
            0 => Err(format_err!("Scene cannot be empty")),
            1 => {
                let leaf = world.remove(0);
                let bounding_box = leaf
                    .bounding_box(time0, time1)
                    .ok_or_else(|| format_err!("Element is missing bounding box"))?;

                Ok(Bvh {
                    tree: Node::Leaf(leaf),
                    bounding_box,
                })
            }
            len => {
                let half = world.drain(len / 2..).collect();
                let (right, left) = if len < MAX_SEQUENTIAL {
                    let right = Bvh::new(half, time0, time1)?;
                    let left = Bvh::new(world, time0, time1)?;

                    (right, left)
                } else {
                    let (right, left) = rayon::join(
                        || Bvh::new(half, time0, time1),
                        || Bvh::new(world, time0, time1),
                    );

                    (right?, left?)
                };
                let bounding_box = surrounding_box(left.bounding_box, right.bounding_box);

                Ok(Bvh {
                    tree: Node::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bounding_box,
                })
            }
        }
    }
}

impl Hittable for Bvh {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(_) = self.bounding_box.hit(r, t_min, t_max) {
            match &self.tree {
                Node::Leaf(object) => object.hit(r, t_min, t_max),
                Node::Branch { left, right } => {
                    let hit_left = left.hit(r, t_min, t_max);
                    let hit_right = {
                        let t_max = hit_left.as_ref().map(|hit| hit.t()).unwrap_or(t_max);
                        right.hit(r, t_min, t_max)
                    };

                    hit_right.or(hit_left)
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
