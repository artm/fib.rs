use std::any::type_name;
use std::ops::{Add, Mul, Sub};
use thiserror::Error;

#[expect(clippy::similar_names, reason = "there is a pattern to madness")]
fn fib<F>(n: usize) -> Result<F, FibError>
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
enum FibError {
    #[error("Fibonacci number F({index}) won't fit in {type_name}")]
    Overflow {
        index: usize,
        type_name: &'static str,
    },
}

enum FibFit {
    Yes,
    No,
    Unknown,
}

trait FibInteger:
    Sized + Copy + Add<Self, Output = Self> + Mul<Self, Output = Self> + Sub<Self, Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn fits_fibonacci(index: usize) -> FibFit;

    fn two() -> Self {
        Self::one() + Self::one()
    }
}

impl FibInteger for u8 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 13 { FibFit::Yes } else { FibFit::No }
    }
}

impl FibInteger for u16 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 24 { FibFit::Yes } else { FibFit::No }
    }
}

impl FibInteger for u32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 47 { FibFit::Yes } else { FibFit::No }
    }
}

impl FibInteger for u64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 93 { FibFit::Yes } else { FibFit::No }
    }
}

impl FibInteger for u128 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 186 {
            FibFit::Yes
        } else {
            FibFit::No
        }
    }
}

mod bigint {
    #![allow(clippy::manual_range_contains)]
    #![allow(clippy::assign_op_pattern)]

    use super::{FibFit, FibInteger};
    use uint::construct_uint;

    construct_uint! {
        pub struct U256(4);
    }

    impl FibInteger for U256 {
        fn zero() -> Self {
            U256::from(0)
        }

        fn one() -> Self {
            U256::from(1)
        }

        fn fits_fibonacci(index: usize) -> FibFit {
            if index <= 250 {
                FibFit::Yes
            } else {
                FibFit::Unknown
            }
        }
    }
}
fn main() {
    let n = 250;
    let fib_n: bigint::U256 = fib(n).unwrap_or_else(|error| panic!("{error}"));
    println!("F({n}) = {fib_n}");
}
