use fib::FibInteger;
use proptest::prelude::*;

fn reference_fibonacci<F>(index: usize) -> F
where
    F: FibInteger,
{
    let mut current = F::zero();
    let mut next = F::one();

    for _ in 1..index {
        (current, next) = (next, current + next);
    }

    if index == 0 { current } else { next }
}

fn test_fibonacci_matches_reference<F>(index: usize)
where
    F: FibInteger + Eq + std::fmt::Debug,
{
    if let Some(max_index) = F::max_fibonacci_index()
        && index > max_index
    {
        assert!(F::fib(index).is_err());
    } else {
        assert_eq!(F::fib(index), Ok(reference_fibonacci(index)));
    }
}

proptest! {
    #[test]
    fn fibonacci_matches_reference_u8(index in 0..200usize) {
        test_fibonacci_matches_reference::<u8>(index);
    }

    #[test]
    fn fibonacci_matches_reference_u16(index in 0..200usize) {
        test_fibonacci_matches_reference::<u16>(index);
    }

    #[test]
    fn fibonacci_matches_reference_u32(index in 0..200usize) {
        test_fibonacci_matches_reference::<u32>(index);
    }

    #[test]
    fn fibonacci_matches_reference_u64(index in 0..200usize) {
        test_fibonacci_matches_reference::<u64>(index);
    }

    #[test]
    fn fibonacci_matches_reference_u128(index in 0..200usize) {
        test_fibonacci_matches_reference::<u128>(index);
    }
}
