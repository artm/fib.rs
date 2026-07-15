use super::{FibFit, FibInteger};

macro_rules! impl_fib_integer {
    (@simple_threshold) => {};
    (@simple_threshold $simple_threshold:expr) => {
        const SIMPLE_THRESHOLD: usize = $simple_threshold;
    };
    ($(($type:ty, $max_index:expr, $lookup:expr $(, $simple_threshold:expr)?)),+ $(,)?) => {
        $(
            impl FibInteger for $type {
                impl_fib_integer!(@simple_threshold $($simple_threshold)?);

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

                fn lookup_size() -> usize {
                    $lookup.len()
                }

                fn lookup(n: usize) -> Option<Self> {
                    $lookup.get(n).copied()
                }
            }
        )+
    };
}

impl_fib_integer! {
    (u8, 13, LOOKUP_U8),
    (u16, 24, LOOKUP_U16),
    (u32, 47, LOOKUP_U32),
    (u64, 93, LOOKUP_U64),
    (u128, 186, LOOKUP_U128, 90),
}

// could have calculated but just listing know values is ok too
const LOOKUP_U8: [u8; 14] = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];

// pre-calculate all values at compile time
const LOOKUP_U16: [u16; 25] = {
    let mut values = [0; 25];
    values[0] = 0;
    values[1] = 1;
    let mut i = 2;
    while i < values.len() {
        values[i] = values[i - 1] + values[i - 2];
        i += 1;
    }
    values
};

const LOOKUP_U32: [u32; 48] = {
    let mut values = [0; 48];
    values[0] = 0;
    values[1] = 1;
    let mut i = 2;
    while i < values.len() {
        values[i] = values[i - 1] + values[i - 2];
        i += 1;
    }
    values
};

const LOOKUP_U64: [u64; 48] = {
    let mut values = [0; 48];
    values[0] = 0;
    values[1] = 1;
    let mut i = 2;
    while i < values.len() {
        values[i] = values[i - 1] + values[i - 2];
        i += 1;
    }
    values
};

const LOOKUP_U128: [u128; 48] = {
    let mut values = [0; 48];
    values[0] = 0;
    values[1] = 1;
    let mut i = 2;
    while i < values.len() {
        values[i] = values[i - 1] + values[i - 2];
        i += 1;
    }
    values
};
