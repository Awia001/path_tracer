use crate::ray::Ray;
use crate::vec3::Vec3;

use Vec3 as Point3;
pub struct Camera {
    pub origin: Point3,
    pub lower_left: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Self {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left + u * self.horizontal + v * self.vertical - self.origin,
        }
    }

    pub fn translate_x(&mut self, by: f64) {
        self.origin.0[0] = (self.origin.x()) + by;
        self.calc_frame_for_origin(self.origin);
    }

    fn calc_frame_for_origin(&mut self, origin: Point3) {
        self.lower_left =
            origin - self.horizontal / 2.0 - self.vertical / 2.0 - Vec3::new(0.0, 0.0, 1.0);
    }
}
