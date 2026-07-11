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

#[cfg(test)]
mod tests {
    use crate::{FibInteger, uint::U256};

    use super::fib;

    fn test_small_well_known<F>()
    where
        F: FibInteger + From<u8> + Eq + std::fmt::Debug,
    {
        let fibonaccis = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (i, expected) in fibonaccis.iter().enumerate() {
            let actual = fib::<F>(i).unwrap();
            assert_eq!(
                F::from(*expected),
                actual,
                "expected F({i}) to be {expected:?}, got {actual:?}"
            );
        }
    }

    #[test]
    fn small_well_known_u8() {
        test_small_well_known::<u8>();
    }
    #[test]
    fn small_well_known_u16() {
        test_small_well_known::<u16>();
    }
    #[test]
    fn small_well_known_u32() {
        test_small_well_known::<u32>();
    }
    #[test]
    fn small_well_known_u64() {
        test_small_well_known::<u64>();
    }
    #[test]
    fn small_well_known_u128() {
        test_small_well_known::<u128>();
    }
    #[test]
    fn small_well_known_u256() {
        test_small_well_known::<U256>();
    }

    fn test_max_index<F>()
    where
        F: FibInteger,
    {
        let Some(max_index) = F::max_fibonacci_index() else {
            return;
        };
        assert!(fib::<F>(max_index).is_ok());
    }

    #[test]
    fn max_index_u8() {
        test_max_index::<u8>();
    }
    #[test]
    fn max_index_u16() {
        test_max_index::<u16>();
    }
    #[test]
    fn max_index_u32() {
        test_max_index::<u32>();
    }
    #[test]
    fn max_index_u64() {
        test_max_index::<u64>();
    }
    #[test]
    fn max_index_u128() {
        test_max_index::<u128>();
    }

    fn test_index_too_large<F>()
    where
        F: FibInteger,
    {
        let Some(max_index) = F::max_fibonacci_index() else {
            return;
        };
        let Some(index) = max_index.checked_add(1) else {
            return;
        };
        assert!(fib::<F>(index).is_err());
    }

    #[test]
    fn index_too_large_u8() {
        test_index_too_large::<u8>();
    }
    #[test]
    fn index_too_large_u16() {
        test_index_too_large::<u16>();
    }
    #[test]
    fn index_too_large_u32() {
        test_index_too_large::<u32>();
    }
    #[test]
    fn index_too_large_u64() {
        test_index_too_large::<u64>();
    }
    #[test]
    fn index_too_large_u128() {
        test_index_too_large::<u128>();
    }
}
