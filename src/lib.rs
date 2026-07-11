#![allow(clippy::missing_errors_doc, reason = "not the focus of this lesson")]

use std::any::type_name;
use std::ops::{Add, Mul, Sub};
use thiserror::Error;

pub mod primint;
pub mod uint;

#[expect(clippy::similar_names, reason = "there is a pattern to madness")]
pub fn fib<F>(n: usize) -> Result<F, FibError>
where
    F: FibInteger,
{
    if n == 0 {
        return Ok(F::zero());
    }
    match F::fits_fibonacci(n) {
        FibFit::No => {
            return Err(FibError::Overflow {
                index: n,
                type_name: type_name::<F>(),
            });
        }
        FibFit::Unknown => {
            eprintln!("todo: checked version");
        }
        FibFit::Yes => {}
    }
    let mut mask: usize = 1 << (usize::BITS - n.leading_zeros() - 1);

    let (mut fib_k, mut fib_k_plus_one) = (F::zero(), F::one());

    loop {
        let fib_2k = fib_k * (F::two() * fib_k_plus_one - fib_k);
        if mask == 1 && n & mask == 0 {
            // n = 2k, we already have the answer, avoid overfloating F(2k + 1)
            return Ok(fib_2k);
        }

        let fib_2k_plus_one = fib_k * fib_k + fib_k_plus_one * fib_k_plus_one;
        if mask == 1 {
            // n = 2k + 1, we already have the answer, avoid overfloating F(2k + 2)
            return Ok(fib_2k_plus_one);
        }

        (fib_k, fib_k_plus_one) = if n & mask == 0 {
            (fib_2k, fib_2k_plus_one)
        } else {
            (fib_2k_plus_one, fib_2k + fib_2k_plus_one)
        };
        mask >>= 1;
    }
}

#[derive(Debug, Error)]
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
    fn fits_fibonacci(index: usize) -> FibFit;

    #[must_use]
    fn two() -> Self {
        Self::one() + Self::one()
    }
}
