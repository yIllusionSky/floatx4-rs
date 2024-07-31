use std::{
    array::IntoIter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    simd::num::SimdFloat as _,
    slice::{Iter, IterMut},
};

use approximately::ApproxEq;

use super::Float;

#[cfg(feature = "f64")]
type SimdFloat = std::simd::f64x4;

#[cfg(not(feature = "f64"))]
pub type SimdFloat = std::simd::f32x4;

#[cfg(feature = "simd")]
#[derive(Clone, Copy, Debug, Default)]
pub struct Floatx4(SimdFloat);

#[cfg(feature = "simd")]
impl Floatx4 {
    /// Created from array.
    pub const fn from_array(array: [Float; 4]) -> Self {
        Self(SimdFloat::from_array(array))
    }

    /// Created from a single element.
    pub fn from_scalar(f: Float) -> Self {
        Self(SimdFloat::splat(f))
    }

    pub fn sum(self) -> Float {
        self.0.reduce_sum()
    }

    pub fn array(&self) -> &[Float; 4] {
        self.0.as_array()
    }

    pub fn mut_array(&mut self) -> &mut [Float; 4] {
        self.0.as_mut_array()
    }

    pub fn iter(&self) -> Iter<Float> {
        self.0.as_array().iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Float> {
        self.0.as_mut_array().iter_mut()
    }

    pub fn copy_from_slice(slice: &[Float]) -> Self {
        Self(SimdFloat::from_slice(slice))
    }
}

impl Add for Floatx4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Floatx4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Floatx4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for Floatx4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl AddAssign for Floatx4 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Floatx4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl MulAssign for Floatx4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl DivAssign for Floatx4 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl IntoIterator for Floatx4 {
    type Item = Float;
    type IntoIter = IntoIter<Float, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_array().into_iter()
    }
}

impl ApproxEq for Floatx4 {
    fn approx<T: std::borrow::Borrow<Self>>(&self, other: T) -> bool {
        self.0.approx(other.borrow().0)
    }
}
