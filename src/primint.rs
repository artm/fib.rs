use crate::FibLookup;

use super::{FibFit, FibInteger, doubling::Doubling, lookup::Lookup, simple::Simple};

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
    (u8, 13, Lookup),
    (u16, 24, Lookup),
    (u32, 47, Lookup),
    (u64, 93, Simple),
    (u128, 186, Doubling),
}

// could have calculated but just listing know values is ok too
const LOOKUP_U8: [u8; 14] = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];

impl FibLookup for u8 {
    fn lookup(n: usize) -> Option<Self> {
        if n < Self::lookup_size() {
            Some(LOOKUP_U8[n])
        } else {
            None
        }
    }

    fn lookup_size() -> usize {
        LOOKUP_U8.len()
    }
}

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

impl FibLookup for u16 {
    fn lookup(n: usize) -> Option<Self> {
        if n < Self::lookup_size() {
            Some(LOOKUP_U16[n])
        } else {
            None
        }
    }

    fn lookup_size() -> usize {
        LOOKUP_U16.len()
    }
}

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

impl FibLookup for u32 {
    fn lookup(n: usize) -> Option<Self> {
        if n < Self::lookup_size() {
            Some(LOOKUP_U32[n])
        } else {
            None
        }
    }

    fn lookup_size() -> usize {
        LOOKUP_U32.len()
    }
}
