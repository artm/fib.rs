use fib::{FibError, FibInteger, doubling, simple};
use proptest::prelude::*;
use std::assert_matches;

fn test_fibonacci_invariants<F>(index: usize)
where
    F: FibInteger + Eq + std::fmt::Debug,
{
    let fast = doubling::fib::<F>(index);
    let simple = simple::fib::<F>(index);
    assert_eq!(fast, simple);

    if index == 0 {
        assert_eq!(fast, Ok(F::zero()));
    } else if index == 1 {
        assert_eq!(fast, Ok(F::one()));
    } else if index <= F::max_fibonacci_index().unwrap() {
        let prev_fast = doubling::fib::<F>(index - 1).unwrap();
        let preprev_fast = doubling::fib::<F>(index - 2).unwrap();
        assert_eq!(fast.unwrap(), prev_fast + preprev_fast);
    } else {
        assert_matches!(fast, Err(FibError::Overflow { .. }));
    }
}

proptest! {
    #[test]
    fn test_fibonacci_invariants_u8(
        index in 0..200usize
    ) {
        test_fibonacci_invariants::<u8>(index);
    }

    #[test]
    fn test_fibonacci_invariants_u16(
        index in 0..200usize
    ) {
        test_fibonacci_invariants::<u16>(index);
    }

    #[test]
    fn test_fibonacci_invariants_u32(
        index in 0..200usize
    ) {
        test_fibonacci_invariants::<u32>(index);
    }

    #[test]
    fn test_fibonacci_invariants_u64(
        index in 0..200usize
    ) {
        test_fibonacci_invariants::<u64>(index);
    }

    #[test]
    fn test_fibonacci_invariants_u128(
        index in 0..200usize
    ) {
        test_fibonacci_invariants::<u128>(index);
    }
}
