use super::{FibError, FibInteger, FibLookup, FibMethod};
use std::any::type_name;

pub struct Lookup;

impl<F> FibMethod<F> for Lookup
where
    F: FibInteger + FibLookup,
{
    fn fib(n: usize) -> Result<F, FibError> {
        if let Some(value) = F::lookup(n) {
            Ok(value)
        } else {
            eprintln!("FIXME this is a wrong error");
            Err(FibError::Overflow {
                index: n,
                type_name: type_name::<F>(),
            })
        }
    }
}
