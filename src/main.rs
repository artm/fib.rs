use fib::{fib, uint};

fn main() {
    let n = 7;
    let fib_n: u8 = fib(n).unwrap_or_else(|error| panic!("{error}"));
    println!("F({n}) = {fib_n}");

    let n = 20;
    let fib_n: u16 = fib(n).unwrap_or_else(|error| panic!("{error}"));
    println!("F({n}) = {fib_n}");

    let n = 40;
    let fib_n: u32 = fib(n).unwrap_or_else(|error| panic!("{error}"));
    println!("F({n}) = {fib_n}");

    let n = 90;
    let fib_n: u64 = fib(n).unwrap_or_else(|error| panic!("{error}"));
    println!("F({n}) = {fib_n}");

    let n = 180;
    let fib_n: u128 = fib(n).unwrap_or_else(|error| panic!("{error}"));
    println!("F({n}) = {fib_n}");

    let n = 250;
    let fib_n: uint::U256 = fib(n).unwrap_or_else(|error| panic!("{error}"));
    println!("F({n}) = {fib_n}");
}
