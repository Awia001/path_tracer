use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.1 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn length_squared(&self) -> f64 {
        num::pow(self.0, 2) + num::pow(self.1, 2) + num::pow(self.2, 2)
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f64> for &mut Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3(0.0, 0.0, 0.0)
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
        Self(-self.0, -self.1, -self.2)
    }
}
