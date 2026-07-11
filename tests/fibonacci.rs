use fib::FibInteger;

type FibFn<F> = fn(usize) -> Result<F, fib::FibError>;

fn test_small_well_known<F>(fib: FibFn<F>)
where
    F: FibInteger + From<u8> + Eq + std::fmt::Debug,
{
    let fibonaccis = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    for (i, expected) in fibonaccis.iter().enumerate() {
        let actual = fib(i).unwrap();
        assert_eq!(
            F::from(*expected),
            actual,
            "expected F({i}) to be {expected:?}, got {actual:?}"
        );
    }
}

fn test_max_index<F>(fib: FibFn<F>)
where
    F: FibInteger,
{
    let Some(max_index) = F::max_fibonacci_index() else {
        return;
    };
    assert!(fib(max_index).is_ok());
}

fn test_index_too_large<F>(fib: FibFn<F>)
where
    F: FibInteger,
{
    let Some(max_index) = F::max_fibonacci_index() else {
        return;
    };
    let Some(index) = max_index.checked_add(1) else {
        return;
    };
    assert!(fib(index).is_err());
}

mod simple {
    use super::{test_index_too_large, test_max_index, test_small_well_known};
    use fib::{simple, uint::U256};

    #[test]
    fn small_well_known_u8() {
        test_small_well_known::<u8>(simple::fib);
    }

    #[test]
    fn small_well_known_u16() {
        test_small_well_known::<u16>(simple::fib);
    }

    #[test]
    fn small_well_known_u32() {
        test_small_well_known::<u32>(simple::fib);
    }

    #[test]
    fn small_well_known_u64() {
        test_small_well_known::<u64>(simple::fib);
    }

    #[test]
    fn small_well_known_u128() {
        test_small_well_known::<u128>(simple::fib);
    }

    #[test]
    fn small_well_known_u256() {
        test_small_well_known::<U256>(simple::fib);
    }

    #[test]
    fn max_index_u8() {
        test_max_index::<u8>(simple::fib);
    }

    #[test]
    fn max_index_u16() {
        test_max_index::<u16>(simple::fib);
    }

    #[test]
    fn max_index_u32() {
        test_max_index::<u32>(simple::fib);
    }

    #[test]
    fn max_index_u64() {
        test_max_index::<u64>(simple::fib);
    }

    #[test]
    fn max_index_u128() {
        test_max_index::<u128>(simple::fib);
    }

    #[test]
    fn index_too_large_u8() {
        test_index_too_large::<u8>(simple::fib);
    }

    #[test]
    fn index_too_large_u16() {
        test_index_too_large::<u16>(simple::fib);
    }

    #[test]
    fn index_too_large_u32() {
        test_index_too_large::<u32>(simple::fib);
    }

    #[test]
    fn index_too_large_u64() {
        test_index_too_large::<u64>(simple::fib);
    }

    #[test]
    fn index_too_large_u128() {
        test_index_too_large::<u128>(simple::fib);
    }
}

mod fast_doubling {
    use super::{test_index_too_large, test_max_index, test_small_well_known};
    use fib::{fib, uint::U256};
    #[test]
    fn small_well_known_u8() {
        test_small_well_known::<u8>(fib);
    }

    #[test]
    fn small_well_known_u16() {
        test_small_well_known::<u16>(fib);
    }

    #[test]
    fn small_well_known_u32() {
        test_small_well_known::<u32>(fib);
    }

    #[test]
    fn small_well_known_u64() {
        test_small_well_known::<u64>(fib);
    }

    #[test]
    fn small_well_known_u128() {
        test_small_well_known::<u128>(fib);
    }

    #[test]
    fn small_well_known_u256() {
        test_small_well_known::<U256>(fib);
    }

    #[test]
    fn max_index_u8() {
        test_max_index::<u8>(fib);
    }

    #[test]
    fn max_index_u16() {
        test_max_index::<u16>(fib);
    }

    #[test]
    fn max_index_u32() {
        test_max_index::<u32>(fib);
    }

    #[test]
    fn max_index_u64() {
        test_max_index::<u64>(fib);
    }

    #[test]
    fn max_index_u128() {
        test_max_index::<u128>(fib);
    }

    #[test]
    fn index_too_large_u8() {
        test_index_too_large::<u8>(fib);
    }

    #[test]
    fn index_too_large_u16() {
        test_index_too_large::<u16>(fib);
    }

    #[test]
    fn index_too_large_u32() {
        test_index_too_large::<u32>(fib);
    }

    #[test]
    fn index_too_large_u64() {
        test_index_too_large::<u64>(fib);
    }

    #[test]
    fn index_too_large_u128() {
        test_index_too_large::<u128>(fib);
    }
}
