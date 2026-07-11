use super::{FibFit, FibInteger};

impl FibInteger for u8 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 13 { FibFit::Yes } else { FibFit::No }
    }
}

impl FibInteger for u16 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 24 { FibFit::Yes } else { FibFit::No }
    }
}

impl FibInteger for u32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 47 { FibFit::Yes } else { FibFit::No }
    }
}

impl FibInteger for u64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 93 { FibFit::Yes } else { FibFit::No }
    }
}

impl FibInteger for u128 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 186 {
            FibFit::Yes
        } else {
            FibFit::No
        }
    }
}
