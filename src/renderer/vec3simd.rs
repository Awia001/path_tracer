use core::simd::{Simd, SimdFloat};
use rand::Rng;
use std::{
    f64,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
    simd::{f64x4, StdFloat},
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3Simd(pub f64x4);

impl Default for Vec3Simd {
    fn default() -> Self {
        Self(Simd::from_array([0., 0., 0., 0.]))
    }
}

impl Vec3Simd {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Simd::from_array([x, y, z, 0.]))
    }

    pub fn dot(&self, other: &Vec3Simd) -> f64 {
        // self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
        Simd::reduce_sum(self.0 * other.0)
    }

    pub fn cross(&self, other: &Vec3Simd) -> Self {
        // Self(Simd::from_array([
        //     self.0[1] * other.0[2] - self.0[2] * other.0[1],
        //     self.0[2] * other.0[0] - self.0[0] * other.0[2],
        //     self.0[0] * other.0[1] - self.0[1] * other.0[0],
        //     0.,
        // ]))

        // Who knows if marshalling to several simd values is faster than the previous approach?
        let arr_a = Simd::as_array(&self.0);
        let arr_b = Simd::as_array(&other.0);

        let arr_left = [
            arr_a[1] * arr_b[2],
            arr_a[2] * arr_b[0],
            arr_a[0] * arr_b[1],
            0.,
        ];
        let arr_right = [
            arr_a[2] * arr_b[1],
            arr_a[0] * arr_b[2],
            arr_a[1] * arr_b[0],
            0.,
        ];

        let simd_a = Simd::from_array(arr_left);
        let simd_b = Simd::from_array(arr_right);
        Self(simd_a - simd_b)
    }

    pub fn length_squared(&self) -> f64 {
        //Simd::reduce_sum(self.0 * self.0)
        num::pow(self.0[0], 2) + num::pow(self.0[1], 2) + num::pow(self.0[2], 2)
    }

    pub fn clamp(&mut self, min: f64, max: f64) {
        let min = Simd::splat(min);
        let max = Simd::splat(max);
        self.0 = self.0.simd_clamp(min, max);
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn unit_vector(&self) -> Self {
        // self / self.length()
        let arr = Simd::as_array(&self.0);
        let length = self.length();
        Self::new(arr[0] / length, arr[1] / length, arr[2] / length)
    }

    pub fn x(&self) -> f64 {
        Simd::as_array(&self.0)[0]
    }

    pub fn y(&self) -> f64 {
        Simd::as_array(&self.0)[1]
    }

    pub fn z(&self) -> f64 {
        Simd::as_array(&self.0)[2]
    }

    pub fn random() -> Vec3Simd {
        Self(Simd::from_array([
            rand::thread_rng().gen(),
            rand::thread_rng().gen(),
            rand::thread_rng().gen(),
            0.,
        ]))
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3Simd {
        Self(Simd::from_array(
            [rand::thread_rng().gen_range(min..max); 4],
        ))
    }

    pub fn random_in_unit_sphere() -> Vec3Simd {
        loop {
            let p = Vec3Simd::random_in_range(-1., 1.);
            if p.length_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3Simd {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3Simd) -> Vec3Simd {
        let in_unit_sphere = Vec3Simd::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0. {
            return in_unit_sphere;
        }
        -in_unit_sphere
    }

    pub fn lerp(&self, b: &Vec3Simd, t: f64) -> Self {
        (1. - t) * self + t * b
    }

    pub fn sqrt(&self) -> Self {
        Self(self.0.sqrt())
    }

    pub fn constrain_colour(&self, samples: u32) -> image::Rgb<u8> {
        // Scale our colour values by how many samples we have taken
        let scale = Self::new(1., 1., 1.) / samples as f64;
        let mut colour = self * scale;

        // Simple gamma correction
        colour = colour.sqrt();

        // Clamp our values between 0. and 0.999 then multiply by 255 to get a value that will fit in a u8
        colour.clamp(0., 0.999);
        colour = colour * 255.;

        let cast_vec = Simd::cast::<u8>(colour.0);

        image::Rgb([cast_vec[0], cast_vec[1], cast_vec[2]])
    }
}

impl Add for Vec3Simd {
    type Output = Vec3Simd;
    fn add(self, rhs: Vec3Simd) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Vec3Simd {
    fn add_assign(&mut self, rhs: Vec3Simd) {
        self.0 += rhs.0;
    }
}

impl Sub for Vec3Simd {
    type Output = Vec3Simd;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Vec3Simd {
    type Output = Vec3Simd;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul for &Vec3Simd {
    type Output = Vec3Simd;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3Simd(self.0 * rhs.0)
    }
}

impl Mul<&Vec3Simd> for Vec3Simd {
    type Output = Vec3Simd;
    fn mul(self, rhs: &Vec3Simd) -> Self::Output {
        Vec3Simd(rhs.0 * self.0)
    }
}

impl Mul<Vec3Simd> for &Vec3Simd {
    type Output = Vec3Simd;
    fn mul(self, rhs: Vec3Simd) -> Self::Output {
        Vec3Simd(rhs.0 * self.0)
    }
}

impl Mul<Vec3Simd> for f64 {
    type Output = Vec3Simd;
    fn mul(self, rhs: Vec3Simd) -> Self::Output {
        let val = f64x4::splat(self);
        Vec3Simd(rhs.0 * val)
    }
}

impl Mul<&Vec3Simd> for f64 {
    type Output = Vec3Simd;
    fn mul(self, rhs: &Vec3Simd) -> Self::Output {
        let val = f64x4::splat(self);
        Vec3Simd(rhs.0 * val)
    }
}

impl Mul<f64> for Vec3Simd {
    type Output = Vec3Simd;
    fn mul(self, rhs: f64) -> Self::Output {
        let val = f64x4::splat(rhs);
        Vec3Simd(val * self.0)
    }
}

impl MulAssign<f64> for &mut Vec3Simd {
    fn mul_assign(&mut self, rhs: f64) {
        let val = f64x4::splat(rhs);
        self.0 *= val;
    }
}

impl Div<f64> for Vec3Simd {
    type Output = Vec3Simd;
    fn div(self, rhs: f64) -> Self::Output {
        1.0 / rhs * self
    }
}

impl Div<f64> for &Vec3Simd {
    type Output = Vec3Simd;
    fn div(self, rhs: f64) -> Self::Output {
        1.0 / rhs * self
    }
}

impl DivAssign<f64> for &mut Vec3Simd {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Neg for Vec3Simd {
    type Output = Vec3Simd;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl From<Vec3Simd> for Simd<f64, 4> {
    fn from(value: Vec3Simd) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::Vec3Simd;

    #[test]
    fn test_dot_product() {
        // Very simple dot product test
        let a = Vec3Simd::new(1.0, 2.0, 3.0);
        let b = Vec3Simd::new(4.0, 5.0, 6.0);

        assert_eq!(32., a.dot(&b));
    }

    #[test]
    fn test_cross_product() {
        let a = Vec3Simd::new(1.0, 2.0, 3.0);
        let b = Vec3Simd::new(4.0, 5.0, 6.0);

        assert_eq!(Vec3Simd::new(-3., 6., -3.), a.cross(&b));
    }
}
