use std::any::type_name;
use std::ops::{Add, Mul, Sub};
use thiserror::Error;

pub mod primint;
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
    // use simple iteration for n in [lookup_size(); SIMPLE_THRESHOLD]
    const SIMPLE_THRESHOLD: usize = usize::MAX;

    fn zero() -> Self;
    fn one() -> Self;
    fn max_fibonacci_index() -> Option<usize>;
    fn fits_fibonacci(index: usize) -> FibFit;

    #[must_use]
    fn two() -> Self {
        Self::one() + Self::one()
    }

    #[must_use]
    fn lookup_size() -> usize {
        2
    }

    #[must_use]
    fn lookup(n: usize) -> Option<Self> {
        match n {
            0 => Some(Self::zero()),
            1 => Some(Self::one()),
            _ => None,
        }
    }

    /// # Errors
    ///
    /// Returns [`FibError::Overflow`] if the requested Fibonacci number does not
    /// fit in `F` according to [`FibInteger::fits_fibonacci`].
    fn fib(n: usize) -> Result<Self, FibError> {
        match Self::fits_fibonacci(n) {
            FibFit::No => {
                return Err(FibError::Overflow {
                    index: n,
                    type_name: type_name::<Self>(),
                });
            }
            FibFit::Unknown => {
                eprintln!("todo: checked version");
            }
            FibFit::Yes => {}
        }

        // look up if index inside lookup table
        if let Some(result) = Self::lookup(n) {
            return Ok(result);
        }

        // set up iterative calculation
        let (stop, mut mask) = if n <= Self::SIMPLE_THRESHOLD {
            // index under threshold - iterate up to F(n - 1), F(n)
            (n - 1, 0)
        } else {
            let mut mask = 1 << (usize::BITS - n.leading_zeros() - 1);
            let mut stop = 0;
            loop {
                let next_stop = stop * 2 + usize::from(n & mask != 0);
                if next_stop > Self::SIMPLE_THRESHOLD {
                    break;
                }
                stop = next_stop;
                mask >>= 1;
            }
            (stop, mask)
        };
        let start = std::cmp::min(Self::lookup_size() - 2, stop);
        let mut fib_k = Self::lookup(start).unwrap();
        let mut fib_k_plus_one = Self::lookup(start + 1).unwrap();
        for _ in start..stop {
            (fib_k, fib_k_plus_one) = (fib_k_plus_one, fib_k + fib_k_plus_one);
        }
        if n <= Self::SIMPLE_THRESHOLD {
            // index under threshold: done
            return Ok(fib_k_plus_one);
        }

        // index above threshold: continue with fast doubling
        loop {
            let fib_2k = fib_k * (Self::two() * fib_k_plus_one - fib_k);
            if mask == 1 && n & mask == 0 {
                // n = 2k, we already have the answer, avoid overflowing F(2k + 1)
                return Ok(fib_2k);
            }

            let fib_2k_plus_one = fib_k * fib_k + fib_k_plus_one * fib_k_plus_one;
            if mask == 1 {
                // n = 2k + 1, we already have the answer, avoid overflowing F(2k + 2)
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
}
