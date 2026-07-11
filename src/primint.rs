use super::{FibFit, FibInteger};

macro_rules! impl_fib_integer {
    ($(($type:ty, $max_index:expr)),+ $(,)?) => {
        $(
            impl FibInteger for $type {
                fn zero() -> Self {
                    0
                }

                fn one() -> Self {
                    1
                }

                fn two() -> Self {
                    2
                }

                fn fits_fibonacci(index: usize) -> FibFit {
                    if index <= $max_index {
                        FibFit::Yes
                    } else {
                        FibFit::No
                    }
                }
            }
        )+
    };
}

impl_fib_integer! {
    (u8, 13),
    (u16, 24),
    (u32, 47),
    (u64, 93),
    (u128, 186),
}
