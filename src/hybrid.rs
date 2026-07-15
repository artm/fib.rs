use super::{FibError, FibFit, FibInteger, FibMethod, simple};
use std::any::type_name;

/// Computes the `n`th Fibonacci number using the hybrid algorithm.
///
/// The algorithm performs $O(\log n)$ arithmetic operations by maintaining
/// the consecutive values `F(k)` and `F(k + 1)` while processing the bits of
/// `n` from most significant to least significant.
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
pub fn fib<F, const THRESHOLD: usize>(n: usize) -> Result<F, FibError>
where
    F: FibInteger,
{
    Hybrid::<THRESHOLD>::fib(n)
}

pub struct Hybrid<const THRESHOLD: usize>;

impl<F, const THRESHOLD: usize> FibMethod<F> for Hybrid<THRESHOLD>
where
    F: FibInteger,
{
    #[expect(
        clippy::similar_names,
        reason = "Though this be madness, yet there is method in 't"
    )]
    fn fib(n: usize) -> Result<F, FibError> {
        if n < THRESHOLD {
            return simple::fib(n);
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
        let mut k = 0;
        while k * 2 + usize::from(n & mask != 0) < THRESHOLD {
            k *= 2;
            k += usize::from(n & mask != 0);
            mask >>= 1;
        }
        let (mut fib_k, mut fib_k_plus_one) = (
            simple::fib(k).expect("fibonacci number with index under threshold are available"),
            simple::fib(k + 1).expect("fibonacci number with index under threshold are available"),
        );

        loop {
            let fib_2k = fib_k * (F::two() * fib_k_plus_one - fib_k);
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
