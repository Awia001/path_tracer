use crate::renderer::Vec3;

type Point3 = Vec3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
        }
    }
}
