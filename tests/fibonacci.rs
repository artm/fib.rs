use fib::{FibInteger, uint::U256};

fn test_small_well_known<F>()
where
    F: FibInteger + From<u8> + Eq + std::fmt::Debug,
{
    let fibonaccis = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    for (i, expected) in fibonaccis.iter().enumerate() {
        let actual = F::fib(i).unwrap();
        assert_eq!(
            F::from(*expected),
            actual,
            "expected F({i}) to be {expected:?}, got {actual:?}"
        );
    }
}

fn test_recurrence<F>(n: usize)
where
    F: FibInteger + Eq + std::fmt::Debug,
{
    assert!(n > 1);
    let (a, b, c) = (
        F::fib(n - 2).unwrap(),
        F::fib(n - 1).unwrap(),
        F::fib(n).unwrap(),
    );
    assert_ne!(c, F::zero());
    assert_eq!(a + b, c, "expected F({n}) to be {:?}, got {c:?}", a + b);
}

fn test_one_past_lookup_table<F>()
where
    F: FibInteger + Eq + std::fmt::Debug,
{
    test_recurrence::<F>(F::lookup_size());
}

fn test_first_fast_double<F>()
where
    F: FibInteger + Eq + std::fmt::Debug,
{
    assert!(F::SIMPLE_THRESHOLD < usize::MAX);
    test_recurrence::<F>(F::SIMPLE_THRESHOLD + 1);
}

fn test_max_index<F>()
where
    F: FibInteger + Eq + std::fmt::Debug,
{
    if let Some(n) = F::max_fibonacci_index() {
        test_recurrence::<F>(n);
    }
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
    assert!(F::fib(index).is_err());
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

#[test]
fn one_past_lookup_table_u64() {
    test_one_past_lookup_table::<u64>();
}

#[test]
fn one_past_lookup_table_u128() {
    test_one_past_lookup_table::<u128>();
}

#[test]
fn one_past_lookup_table_u256() {
    test_one_past_lookup_table::<U256>();
}

#[test]
fn first_fast_double_u128() {
    test_first_fast_double::<u128>();
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
