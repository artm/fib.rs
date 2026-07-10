use std::any::type_name;
use thiserror::Error;

#[expect(clippy::similar_names, reason = "there is a pattern to madness")]
fn fib<F>(n: u8) -> Result<F, FibError>
where
    F: num_traits::PrimInt + num_traits::Unsigned + MaxFibIndex,
{
    if n == 0 {
        return Ok(F::zero());
    }
    if n > F::MAX_FIB_INDEX {
        return Err(FibError::Overflow {
            index: n,
            type_name: type_name::<F>(),
        });
    }
    let two = F::from(2).expect("there can always be 2");
    let mut mask = 1 << (u8::BITS - n.leading_zeros() - 1);

    let (mut fib_k, mut fib_k_plus_one) = (F::zero(), F::one());

    loop {
        let fib_2k = fib_k * (two * fib_k_plus_one - fib_k);
        if mask == 1 && n & mask == 0 {
            // n = 2k, we already have the answer, avoid overfloating F(2k + 1)
            return Ok(fib_2k);
        }

        let fib_2k_plus_one = fib_k * fib_k + fib_k_plus_one * fib_k_plus_one;
        if mask == 1 {
            // n = 2k + 1, we already have the answer, avoid overfloating F(2k + 2)
            return Ok(fib_2k_plus_one);
        }

        (fib_k, fib_k_plus_one) = if n & mask == 0 {
            (fib_2k, fib_2k_plus_one)
        } else {
            (fib_2k_plus_one, fib_2k + fib_2k_plus_one)
        };
        mask >>= 1;
    }
}

#[derive(Debug, Error)]
enum FibError {
    #[error("Fibonacci number F({index}) won't fit in {type_name}")]
    Overflow { index: u8, type_name: &'static str },
}

trait MaxFibIndex {
    const MAX_FIB_INDEX: u8;
}

impl MaxFibIndex for u8 {
    const MAX_FIB_INDEX: u8 = 13;
}

impl MaxFibIndex for u16 {
    const MAX_FIB_INDEX: u8 = 24;
}

impl MaxFibIndex for u32 {
    const MAX_FIB_INDEX: u8 = 47;
}

impl MaxFibIndex for u64 {
    const MAX_FIB_INDEX: u8 = 93;
}

impl MaxFibIndex for u128 {
    const MAX_FIB_INDEX: u8 = 186;
}

fn main() {
    use num_format::{Locale, ToFormattedString};

    let n = u128::MAX_FIB_INDEX + 1;
    let fib_n: u128 = fib(n).unwrap_or_else(|error| panic!("{error}"));
    println!("F({n}) = {}", fib_n.to_formatted_string(&Locale::en));
}
