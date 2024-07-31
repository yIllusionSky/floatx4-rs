//! Used to express the combination of four float numbers.
//!
//! # Examples
//!
//! ```rust
//! use floatx4::Floatx4;
//!
//! // let array = Floatx4::from_scalar(2.);
//! let array = Floatx4::from_array([1.,2.,3.,4.]);
//!
//! // +,-,*,/ ...
//! let mut new_array = array * array ;
//! new_array+=array;
//!
//! // print
//! for value in new_array{
//!     println!("{value:?}");
//! }
//!
//! ```
#![feature(portable_simd)]

#[cfg(feature = "simd")]
#[path = "simd.rs"]
mod array;

#[cfg(not(feature = "simd"))]
mod array;

#[cfg(feature = "f64")]
pub type Float = f64;

#[cfg(not(feature = "f64"))]
pub type Float = f32;

pub use array::Floatx4;

#[cfg(test)]
mod tests {
    use approximately::ApproxEq;

    use crate::Floatx4;
    #[test]
    fn test_floatx4() {
        let first = Floatx4::from_array([1., 2., 3., 4.]);
        let second = Floatx4::from_scalar(2.);
        (first + second).assert_approx(Floatx4::from_array([3., 4., 5., 6.]));
        (first - second).assert_approx(Floatx4::from_array([-1., 0., 1., 2.]));
        (first * second).assert_approx(Floatx4::from_array([2., 4., 6., 8.]));
        (first / second).assert_approx(Floatx4::from_array([0.5, 1., 1.5, 2.]));
    }
}
