use super::{FibError, FibFit, FibInteger};
use std::any::type_name;

#[expect(clippy::missing_errors_doc)]
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
    let mut last_fib: F = F::zero();
    let mut fib: F = F::one();

    for _ in 0..(n - 1) {
        (last_fib, fib) = (fib, last_fib + fib);
    }

    Ok(fib)
}
