use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
