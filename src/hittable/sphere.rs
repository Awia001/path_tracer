use crate::hittable::{HitRecord, Hittable};
use crate::renderer::Ray;

type Vec3 = nalgebra::Vector3<f64>;
/// A sphere Hittable that defines it's centre point and radius
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f64) -> Self {
        Self { centre, radius }
    }
}

impl Hittable for Sphere {
    /// impl of the Hittable trait for the Sphere
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.orig - self.centre;
        let a = ray.dir.magnitude_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = f64::sqrt(discriminant);
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            };
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.centre) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::hittable::{HitRecord, Hittable, Sphere};
    use crate::renderer::Ray;

    type Vec3 = nalgebra::Vector3<f64>;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere {
            centre: Vec3::new(0., 0., -2.),
            radius: 1.5,
        };
        let ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., -1.));
        let mut hit_record = HitRecord {
            ..Default::default()
        };

        let hit = sphere.hit(&ray, 0.0001, f64::MAX, &mut hit_record);
        assert!(hit);
    }
}
