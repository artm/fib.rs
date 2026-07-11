use std::any::type_name;
use thiserror::Error;

#[expect(clippy::similar_names, reason = "there is a pattern to madness")]
fn fib<F>(n: usize) -> Result<F, FibError>
where
    F: num_traits::PrimInt + num_traits::Unsigned + FibInteger,
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
    let mut mask: usize = 1 << (usize::BITS - n.leading_zeros() - 1);

    let (mut fib_k, mut fib_k_plus_one) = (F::zero(), F::one());

    loop {
        let fib_2k = fib_k * (F::TWO * fib_k_plus_one - fib_k);
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
    Overflow {
        index: usize,
        type_name: &'static str,
    },
}

trait FibInteger {
    const MAX_FIB_INDEX: usize;
    const TWO: Self;
}

impl FibInteger for u8 {
    const MAX_FIB_INDEX: usize = 13;
    const TWO: Self = 2;
}

impl FibInteger for u16 {
    const MAX_FIB_INDEX: usize = 24;
    const TWO: Self = 2;
}

impl FibInteger for u32 {
    const MAX_FIB_INDEX: usize = 47;
    const TWO: Self = 2;
}

impl FibInteger for u64 {
    const MAX_FIB_INDEX: usize = 93;
    const TWO: Self = 2;
}

impl FibInteger for u128 {
    const MAX_FIB_INDEX: usize = 186;
    const TWO: Self = 2;
}

fn main() {
    use num_format::{Locale, ToFormattedString};

    let n = u128::MAX_FIB_INDEX;
    let fib_n: u128 = fib(n).unwrap_or_else(|error| panic!("{error}"));
    println!("F({n}) = {}", fib_n.to_formatted_string(&Locale::en));
}
