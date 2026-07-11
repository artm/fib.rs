use fib::{fib, uint};

fn main() {
    let n = 250;
    let fib_n: uint::U256 = fib(n).unwrap_or_else(|error| panic!("{error}"));
    println!("F({n}) = {fib_n}");
}
