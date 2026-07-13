#![allow(clippy::manual_range_contains)]
#![allow(clippy::assign_op_pattern)]

use super::{FibFit, FibInteger, doubling::Doubling};
use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

impl FibInteger for U256 {
    type Method = Doubling;

    fn zero() -> Self {
        U256::from(0)
    }

    fn one() -> Self {
        U256::from(1)
    }

    fn max_fibonacci_index() -> Option<usize> {
        None
    }

    fn fits_fibonacci(index: usize) -> FibFit {
        if index <= 250 {
            FibFit::Yes
        } else {
            FibFit::Unknown
        }
    }
}
