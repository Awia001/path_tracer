use crate::hittable::{HitRecord, Hittable};
use crate::renderer::Ray;

use std::vec::Vec;

pub struct HittableList {
    world: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { world: Vec::new() }
    }

    pub fn add_hittable(&mut self, hittable: Box<dyn Hittable>) {
        self.world.push(hittable);
    }

    pub fn from_file(filename: String) {}
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec: HitRecord = HitRecord {
            ..Default::default()
        };

        for hittable in &self.world {
            if hittable.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
