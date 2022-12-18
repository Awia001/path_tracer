use num::{Float, Integer, Num};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Float + Clone + Copy> Vec3<T> {
    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3<T>) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.1 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn length_squared(&self) -> T {
        num::pow(self.0, 2) + num::pow(self.1, 2) + num::pow(self.2, 2)
    }

    pub fn length(&self) -> T {
        T::sqrt(self.length_squared())
    }
}

impl<T: Add<Output = T>> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        *self = Self(self.0 += rhs.0, self.1 += rhs.1, self.2 += rhs.2)
    }
}

impl<T: Mul<Output = T>> Mul<f64> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl<T: Num + MulAssign + Copy> MulAssign for &mut Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = Self(self.0 *= rhs, self.1 *= rhs, self.2 *= rhs)
    }
}

impl<T: Float + MulAssign + Copy> DivAssign for Vec3<T>
// where
//     f64: Div<T>,
{
    fn div_assign(&mut self, rhs: f64) {
        self *= (1 as f64) / rhs;
    }
}

impl<T: Num + std::ops::Neg<Output = T>> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}
