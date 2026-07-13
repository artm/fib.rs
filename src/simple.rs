use super::{FibError, FibFit, FibInteger, FibMethod};
use std::any::type_name;

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
pub fn fib<F>(n: usize) -> Result<F, FibError>
where
    F: FibInteger,
{
    Simple::fib(n)
}

pub struct Simple;

impl<F> FibMethod<F> for Simple
where
    F: FibInteger,
{
    fn fib(n: usize) -> Result<F, FibError> {
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
        let mut last_fib: F = F::zero();
        let mut fib: F = F::one();

        for _ in 0..(n - 1) {
            (last_fib, fib) = (fib, last_fib + fib);
        }

        Ok(fib)
    }
}
