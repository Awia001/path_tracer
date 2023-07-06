use rand::Rng;
use std::{
    f64,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

#[derive(Copy, Clone, PartialEq, Debug)]
//pub struct Vec3(pub f64x4);
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
        // Simd::reduce_sum(self.0 * other.0)
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }

        // Who knows if marshalling to several simd values is faster than the previous approach?
        // let arr_a = Simd::as_array(&self.0);
        // let arr_b = Simd::as_array(&other.0);

        // let arr_left = [
        //     arr_a[1] * arr_b[2],
        //     arr_a[2] * arr_b[0],
        //     arr_a[0] * arr_b[1],
        //     0.,
        // ];
        // let arr_right = [
        //     arr_a[2] * arr_b[1],
        //     arr_a[0] * arr_b[2],
        //     arr_a[1] * arr_b[0],
        //     0.,
        // ];

        // let simd_a = Simd::from_array(arr_left);
        // let simd_b = Simd::from_array(arr_right);
        // Self(simd_a - simd_b)
    }

    pub fn length_squared(&self) -> f64 {
        //Simd::reduce_sum(self.0 * self.0)
        num::pow(self.x, 2) + num::pow(self.y, 2) + num::pow(self.z, 2)
    }

    pub fn clamp(&mut self, min: f64, max: f64) {
        self.x = self.x.clamp(min, max);
        self.y = self.y.clamp(min, max);
        self.z = self.z.clamp(min, max);
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn unit_vector(&self) -> Self {
        // self / self.length()
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn random() -> Self {
        Self {
            x: rand::thread_rng().gen(),
            y: rand::thread_rng().gen(),
            z: rand::thread_rng().gen(),
        }
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        Self {
            x: rand::thread_rng().gen_range(min..max),
            y: rand::thread_rng().gen_range(min..max),
            z: rand::thread_rng().gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in_range(-1., 1.);
            if p.length_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0. {
            return in_unit_sphere;
        }
        -in_unit_sphere
    }

    pub fn lerp(&self, b: &Self, t: f64) -> Self {
        (1. - t) * self + t * b
    }

    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    // pub fn as_array(&self) -> &[f64; 3] {
    //     &[self.x, self.y, self.z]
    // }

    pub fn constrain_colour(&self, samples: u64) -> image::Rgb<u8> {
        // Scale our colour values by how many samples we have taken
        let scale = Self {
            x: 1.,
            y: 1.,
            z: 1.,
        } / samples as f64;
        let mut colour = self * scale;

        // Simple gamma correction
        colour = colour.sqrt();

        // Clamp our values between 0. and 0.999 then multiply by 255 to get a value that will fit in a u8
        colour.clamp(0., 0.999);
        colour = colour * 255.;

        let cast_vec: [u8; 3] = [colour.x as u8, colour.y as u8, colour.z as u8];

        image::Rgb([cast_vec[0], cast_vec[1], cast_vec[2]])
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// impl Mul for &Vec3 {
//     type Output = Self;
//     fn mul(self, rhs: Self) -> Self::Output {
//         Vec3 {
//             x: self.x * rhs.x,
//             y: self.y * rhs.y,
//             z: self.z * rhs.z,
//         }
//     }
// }

impl Mul<&Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for &mut Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        1.0 / rhs * self
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        1.0 / rhs * self
    }
}

impl DivAssign<f64> for &mut Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::Vec3;

    #[test]
    fn test_dot_product() {
        // Very simple dot product test
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(32., a.dot(&b));
    }

    #[test]
    fn test_cross_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(Vec3::new(-3., 6., -3.), a.cross(&b));
    }
}
