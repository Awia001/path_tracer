use crate::hittable::HitRecord;
use crate::renderer::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
