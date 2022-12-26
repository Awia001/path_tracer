use core::simd::{Simd, SimdFloat};
use std::{
    f64,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
    simd::f64x4,
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3(pub f64x4);

impl Default for Vec3 {
    fn default() -> Self {
        Self(Simd::from_array([0., 0., 0., 0.]))
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Simd::from_array([x, y, z, 0.]))
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        // self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
        Simd::reduce_sum(self.0 * other.0)
    }

    pub fn cross(&self, other: &Vec3) -> Self {
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

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
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
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let val = f64x4::splat(self);
        Vec3(rhs.0 * val)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        let val = f64x4::splat(rhs);
        Vec3(val * self.0)
    }
}

impl MulAssign<f64> for &mut Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        let val = f64x4::splat(rhs);
        self.0 *= val;
    }
}

impl Div<f64> for Vec3 {
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
        Self(-self.0)
    }
}

impl From<Vec3> for Simd<f64, 4> {
    fn from(value: Vec3) -> Self {
        Self::from(value.0)
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
