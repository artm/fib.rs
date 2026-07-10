use std::any::type_name;

#[expect(clippy::similar_names, reason = "there is a pattern to madness")]
fn fib<F>(n: u8) -> F
where
    F: num_traits::PrimInt + num_traits::Unsigned + MaxFibIndex,
{
    assert!(
        n <= F::MAX_FIB_INDEX,
        "Fibonacci number F({n}) won't fit in {}",
        type_name::<F>()
    );
    let two = F::from(2).expect("there can always be 2");
    let mut mask = 1 << (u8::BITS - n.leading_zeros() - 1);

    // F(k)     = fib_k
    // F(k + 1) = fib_k_1
    let (mut fib_k, mut fib_k_1) = (F::zero(), F::one());

    loop {
        // F(2k) = fib_2k
        let fig_2k = fib_k * (two * fib_k_1 - fib_k);
        if mask == 1 && n & mask == 0 {
            // n = 2k, we already have the answer, avoid overfloating F(2k + 1)
            return fig_2k;
        }

        // F(2k + 1) = fib_2k_1
        let fib_2k_1 = fib_k * fib_k + fib_k_1 * fib_k_1;
        if mask == 1 {
            // n = 2k + 1, we already have the answer, avoid overfloating F(2k + 2)
            return fib_2k_1;
        }

        (fib_k, fib_k_1) = if n & mask == 0 {
            (fig_2k, fib_2k_1)
        } else {
            (fib_2k_1, fig_2k + fib_2k_1)
        };
        mask >>= 1;
    }
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

    let n = 14;
    let fib_n: u8 = fib(n);
    println!("F({n}) = {}", fib_n.to_formatted_string(&Locale::en));
}
