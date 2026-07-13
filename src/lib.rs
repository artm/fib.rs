use std::ops::{Add, Mul, Sub};
use thiserror::Error;

pub mod doubling;
pub mod primint;
pub mod simple;
pub mod uint;

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
    fn zero() -> Self;
    fn one() -> Self;
    fn max_fibonacci_index() -> Option<usize>;
    fn fits_fibonacci(index: usize) -> FibFit;

    #[must_use]
    fn two() -> Self {
        Self::one() + Self::one()
    }
}
