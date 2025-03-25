use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone)]
pub struct Vec3 {
    pub elements: [f64; 3],
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[x:{}, y:{}, z:{}]", self.x(), self.y(), self.z())
    }
}

// 运算符重载[]
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

// 运算符重载-
impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            elements: [-self.elements[0], -self.elements[1], -self.elements[2]],
        }
    }
}

// 运算符重载 + +=
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            elements: [
                self.elements[0] + rhs.elements[0],
                self.elements[1] + rhs.elements[1],
                self.elements[2] + rhs.elements[2],
            ],
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.elements[0] += rhs.elements[0];
        self.elements[1] += rhs.elements[1];
        self.elements[2] += rhs.elements[2];
    }
}

// 运算符重载 - -=
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_with_value(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.elements[0] -= rhs.elements[0];
        self.elements[1] -= rhs.elements[1];
        self.elements[2] -= rhs.elements[2];
    }
}

// 运算符重载 * *=
/// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
/// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            elements: [
                self.elements[0] * rhs,
                self.elements[1] * rhs,
                self.elements[2] * rhs,
            ],
        }
    }
}
/// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            elements: [
                self.elements[0] * rhs.elements[0],
                self.elements[1] * rhs.elements[1],
                self.elements[2] * rhs.elements[2],
            ],
        }
    }
}
/// Vec3 *= Vec3
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.elements[0] *= rhs.elements[0];
        self.elements[1] *= rhs.elements[1];
        self.elements[2] *= rhs.elements[2];
    }
}

// 运算符重载 / /=
/// Vec3 / Vec3
impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            elements: [
                self.elements[0] / rhs.elements[0],
                self.elements[1] / rhs.elements[1],
                self.elements[2] / rhs.elements[2],
            ],
        }
    }
}
/// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            elements: [
                self.elements[0] / rhs,
                self.elements[1] / rhs,
                self.elements[2] / rhs,
            ],
        }
    }
}
/// Vec3 /= Vec3
impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.elements[0] /= rhs.elements[0];
        self.elements[1] /= rhs.elements[1];
        self.elements[2] /= rhs.elements[2];
    }
}

impl Vec3 {
    pub fn new() -> Self {
        Self { elements: [0.0; 3] }
    }
    pub fn new_with_value(x: f64, y: f64, z: f64) -> Self {
        Self {
            elements: [x, y, z],
        }
    }
    pub fn x(&self) -> f64 {
        self.elements[0]
    }
    pub fn y(&self) -> f64 {
        self.elements[1]
    }
    pub fn z(&self) -> f64 {
        self.elements[2]
    }
    pub fn length_squared(&self) -> f64 {
        self.elements[0] * self.elements[0]
            + self.elements[1] * self.elements[1]
            + self.elements[2] * self.elements[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unite_vector(v: Self) -> Self {
        v.clone() / v.length()
    }
    pub fn dot(u: &Self, v: &Self) -> f64 {
        u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
    }
    pub fn cross(u: &Self, v: &Self) -> Self {
        Self::new_with_value(
            u.y() * v.z() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.x() * v.y(),
        )
    }
}
