use crate::renderer::{Ray, Vec3};

type Point3 = Vec3;

/// A record of where a Ray intersected with a Hittable, will be used for reflection, refraction, shading and much more!
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }
}

impl HitRecord {
    /// Determine if we hit the front or back face of a hittable
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}
