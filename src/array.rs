#![allow(clippy::needless_range_loop)]
use std::{
    array::IntoIter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    slice::{Iter, IterMut},
};

use approximately::ApproxEq;

use super::Float;

#[derive(Clone, Copy, Debug, Default)]
pub struct Floatx4([Float; 4]);

#[cfg(not(feature = "simd"))]
impl Floatx4 {
    /// Created from array.
    pub const fn from_array(array: [Float; 4]) -> Self {
        Self(array)
    }

    /// Created from a single element.
    pub fn from_scalar(f: Float) -> Self {
        Self([f; 4])
    }

    pub fn sum(self) -> Float {
        self.0.into_iter().sum()
    }

    pub fn array(&self) -> &[Float; 4] {
        &self.0
    }

    pub fn mut_array(&mut self) -> &mut [Float; 4] {
        &mut self.0
    }

    pub fn iter(&self) -> Iter<Float> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Float> {
        self.0.iter_mut()
    }
    pub fn copy_from_slice(slice: &[Float]) -> Self {
        Self(slice.try_into().unwrap())
    }
}

impl Add for Floatx4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_value = self.0;
        for index in 0..4 {
            new_value[index] += rhs.0[index];
        }
        Self(new_value)
    }
}

impl Sub for Floatx4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut new_value = self.0;
        for index in 0..4 {
            new_value[index] -= rhs.0[index];
        }
        Self(new_value)
    }
}

impl Mul for Floatx4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_value = self.0;
        for index in 0..4 {
            new_value[index] *= rhs.0[index];
        }
        Self(new_value)
    }
}

impl Div for Floatx4 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut new_value = self.0;
        for index in 0..4 {
            new_value[index] /= rhs.0[index];
        }
        Self(new_value)
    }
}

impl AddAssign for Floatx4 {
    fn add_assign(&mut self, rhs: Self) {
        for index in 0..4 {
            self.0[index] += rhs.0[index];
        }
    }
}

impl SubAssign for Floatx4 {
    fn sub_assign(&mut self, rhs: Self) {
        for index in 0..4 {
            self.0[index] -= rhs.0[index];
        }
    }
}

impl MulAssign for Floatx4 {
    fn mul_assign(&mut self, rhs: Self) {
        for index in 0..4 {
            self.0[index] *= rhs.0[index];
        }
    }
}
impl DivAssign for Floatx4 {
    fn div_assign(&mut self, rhs: Self) {
        for index in 0..4 {
            self.0[index] /= rhs.0[index];
        }
    }
}

impl IntoIterator for Floatx4 {
    type Item = Float;
    type IntoIter = IntoIter<Float, 4>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl ApproxEq for Floatx4 {
    fn approx<T: std::borrow::Borrow<Self>>(&self, other: T) -> bool {
        self.0.approx(other.borrow().0)
    }
}
