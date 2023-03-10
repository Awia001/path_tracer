use crate::hittable::{HitRecord, Hittable};
use crate::renderer::{Ray, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

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
        rec.p = ray.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::hittable::{HitRecord, Hittable, Sphere};
    use crate::renderer::{Ray, Vec3};

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere {
            center: Vec3::new(0., 0., -2.),
            radius: 1.5,
        };
        let ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., -1.));
        let mut hit_record = HitRecord {
            ..Default::default()
        };

        let hit = sphere.hit(&ray, 0., f64::INFINITY, &mut hit_record);
        assert_eq!(hit, true);
    }
}
