use super::{FibFit, FibInteger, doubling::Doubling, simple::Simple};

macro_rules! impl_fib_integer {
    ($(($type:ty, $max_index:expr, $method:ty)),+ $(,)?) => {
        $(
            impl FibInteger for $type {
                type Method = $method;

                fn zero() -> Self {
                    0
                }

                fn one() -> Self {
                    1
                }

                fn two() -> Self {
                    2
                }

                fn max_fibonacci_index() -> Option<usize> {
                    Some($max_index)
                }

                fn fits_fibonacci(index: usize) -> FibFit {
                    if index <= Self::max_fibonacci_index().expect("primitive unsigned integers know their max index") {
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
    (u8, 13, Simple),
    (u16, 24, Simple),
    (u32, 47, Simple),
    (u64, 93, Simple),
    (u128, 186, Doubling),
}
