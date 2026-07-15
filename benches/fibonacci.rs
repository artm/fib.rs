use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fib::{FibInteger, uint::U256};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let max_indexes = [
        <u8 as FibInteger>::max_fibonacci_index().unwrap(),
        <u16 as FibInteger>::max_fibonacci_index().unwrap(),
        <u32 as FibInteger>::max_fibonacci_index().unwrap(),
        <u64 as FibInteger>::max_fibonacci_index().unwrap(),
        <u128 as FibInteger>::max_fibonacci_index().unwrap(),
    ];

    for n in max_indexes {
        if n <= <u8 as FibInteger>::max_fibonacci_index().unwrap() {
            c.bench_with_input(BenchmarkId::new("fib<u8>", n), &n, |b, &n| {
                b.iter(|| <u8 as FibInteger>::fib(black_box(n)));
            });
        }
        if n <= <u16 as FibInteger>::max_fibonacci_index().unwrap() {
            c.bench_with_input(BenchmarkId::new("fib<u16>", n), &n, |b, &n| {
                b.iter(|| <u16 as FibInteger>::fib(black_box(n)));
            });
        }
        if n <= <u32 as FibInteger>::max_fibonacci_index().unwrap() {
            c.bench_with_input(BenchmarkId::new("fib<u32>", n), &n, |b, &n| {
                b.iter(|| <u32 as FibInteger>::fib(black_box(n)));
            });
        }
        if n <= <u64 as FibInteger>::max_fibonacci_index().unwrap() {
            c.bench_with_input(BenchmarkId::new("fib<u64>", n), &n, |b, &n| {
                b.iter(|| <u64 as FibInteger>::fib(black_box(n)));
            });
        }
        if n <= <u128 as FibInteger>::max_fibonacci_index().unwrap() {
            c.bench_with_input(BenchmarkId::new("fib<u128>", n), &n, |b, &n| {
                b.iter(|| <u128 as FibInteger>::fib(black_box(n)));
            });
        }
        // FIXME find and formalize max index for U256
        c.bench_with_input(BenchmarkId::new("fib<U256>", n), &n, |b, &n| {
            b.iter(|| <U256 as FibInteger>::fib(black_box(n)));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
