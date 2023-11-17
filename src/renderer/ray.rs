type Point3 = nalgebra::Vector3<f64>;
type Vec3 = nalgebra::Vector3<f64>;
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
