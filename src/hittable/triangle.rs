use crate::hittable::{HitRecord, Hittable};

use crate::renderer::{Ray, Vec3};

use Vec3 as Point3;
pub struct Triangle {
    verticies: [Point3; 3],
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        false
    }
}
