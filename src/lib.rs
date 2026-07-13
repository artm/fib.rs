use std::ops::{Add, Mul, Sub};
use thiserror::Error;

pub mod doubling;
pub mod primint;
pub mod simple;
pub mod uint;

/// Computes the `n`th Fibonacci number
///
/// Lets output type determine the appropriate method
///
/// Returns [`FibError::Overflow`] when `F` reports that the requested value
/// cannot fit. A [`FibFit::Unknown`] result currently continues with the
/// unchecked calculation and may therefore overflow according to `F`'s
/// arithmetic behavior.
///
/// # Errors
///
/// Returns [`FibError::Overflow`] if the requested Fibonacci number does not
/// fit in `F` according to [`FibInteger::fits_fibonacci`].
pub fn fib<F>(n: usize) -> Result<F, FibError>
where
    F: FibInteger,
{
    F::Method::fib(n)
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FibError {
    #[error("Fibonacci number F({index}) won't fit in {type_name}")]
    Overflow {
        index: usize,
        type_name: &'static str,
    },
}

pub enum FibFit {
    Yes,
    No,
    Unknown,
}

pub trait FibInteger:
    Sized + Copy + Add<Self, Output = Self> + Mul<Self, Output = Self> + Sub<Self, Output = Self>
{
    type Method: FibMethod<Self>;

    fn zero() -> Self;
    fn one() -> Self;
    fn max_fibonacci_index() -> Option<usize>;
    fn fits_fibonacci(index: usize) -> FibFit;

    #[must_use]
    fn two() -> Self {
        Self::one() + Self::one()
    }
}

pub trait FibMethod<F>
where
    F: FibInteger,
{
    /// Computes the `n`th Fibonacci number
    ///
    /// Returns [`FibError::Overflow`] when `F` reports that the requested value
    /// cannot fit. A [`FibFit::Unknown`] result currently continues with the
    /// unchecked calculation and may therefore overflow according to `F`'s
    /// arithmetic behavior.
    ///
    /// # Errors
    ///
    /// Returns [`FibError::Overflow`] if the requested Fibonacci number does not
    /// fit in `F` according to [`FibInteger::fits_fibonacci`].
    fn fib(n: usize) -> Result<F, FibError>;
}
