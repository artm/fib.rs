use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fib::{FibInteger, fib, simple, uint::U256};
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
            c.bench_with_input(BenchmarkId::new("simple::fib<u8>", n), &n, |b, &n| {
                b.iter(|| simple::fib::<u8>(black_box(n)));
            });
            c.bench_with_input(BenchmarkId::new("fib<u8>", n), &n, |b, &n| {
                b.iter(|| fib::<u8>(black_box(n)));
            });
        }
        if n <= <u16 as FibInteger>::max_fibonacci_index().unwrap() {
            c.bench_with_input(BenchmarkId::new("simple::fib<u16>", n), &n, |b, &n| {
                b.iter(|| simple::fib::<u16>(black_box(n)));
            });
            c.bench_with_input(BenchmarkId::new("fib<u16>", n), &n, |b, &n| {
                b.iter(|| fib::<u16>(black_box(n)));
            });
        }
        if n <= <u32 as FibInteger>::max_fibonacci_index().unwrap() {
            c.bench_with_input(BenchmarkId::new("simple::fib<u32>", n), &n, |b, &n| {
                b.iter(|| simple::fib::<u32>(black_box(n)));
            });
            c.bench_with_input(BenchmarkId::new("fib<u32>", n), &n, |b, &n| {
                b.iter(|| fib::<u32>(black_box(n)));
            });
        }
        if n <= <u64 as FibInteger>::max_fibonacci_index().unwrap() {
            c.bench_with_input(BenchmarkId::new("simple::fib<u64>", n), &n, |b, &n| {
                b.iter(|| simple::fib::<u64>(black_box(n)));
            });
            c.bench_with_input(BenchmarkId::new("fib<u64>", n), &n, |b, &n| {
                b.iter(|| fib::<u64>(black_box(n)));
            });
        }
        if n <= <u128 as FibInteger>::max_fibonacci_index().unwrap() {
            c.bench_with_input(BenchmarkId::new("simple::fib<u128>", n), &n, |b, &n| {
                b.iter(|| simple::fib::<u128>(black_box(n)));
            });
            c.bench_with_input(BenchmarkId::new("fib<u128>", n), &n, |b, &n| {
                b.iter(|| fib::<u128>(black_box(n)));
            });
        }
        // FIXME find and formalize max index for U256
        c.bench_with_input(BenchmarkId::new("simple::fib<U256>", n), &n, |b, &n| {
            b.iter(|| simple::fib::<U256>(black_box(n)));
        });
        c.bench_with_input(BenchmarkId::new("fib<U256>", n), &n, |b, &n| {
            b.iter(|| fib::<U256>(black_box(n)));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
