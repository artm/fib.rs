# TODO

This project is being developed as a learning exercise. The following plans are ordered roughly from foundational validation to broader experimentation.

## Testing

- [x] Add tests for every currently supported integer type.
  - Verify known Fibonacci values.
  - Test `F(0)` and other small boundary cases.
  - Test the largest index known to fit.
  - Test the first index that does not fit.

- [x] Add a simpler iterative Fibonacci implementation to use as a reference implementation.
  - Keep it straightforward rather than optimized.
  - Compare the fast-doubling implementation against it for values that fit.

- [x] Add fuzz tests.
  - Compare fast doubling with the simpler iterative implementation across many indices and supported types.
  - Test various combinations of integer types and indices.
  - Check useful invariants, including:
    - `F(0) = 0` and `F(1) = 1`.
    - `F(n + 2) = F(n) + F(n + 1)` where the values fit.
    - Fast-doubling results agree with the reference implementation.
    - A reported `No` fit result is not used to produce a value.

## Fit information and type abstraction

- [x] Add a way for a type to report its maximum Fibonacci index as `Option<usize>`.
  - `Some(index)` represents a known maximum index.
  - `None` represents an unbounded type or a type whose maximum is not known statically.

- [x] Derive `fits_fibonacci()` from the optional maximum-index information where appropriate.
  - Preserve the ability for types with more complex rules to provide their own implementation.
  - Decide how `Unknown` relates to `None` and to checked arithmetic.

## Performance

- [x] Add a benchmarking harness.
  - Compare the simple iterative implementation with fast doubling.
  - Measure supported types separately.
  - Include larger indices where the types can represent the result.
  - Record whether arithmetic cost or algorithmic complexity dominates.

- [ ] Restore and expand property-based tests for the unified implementation.
  - Compare `FibInteger::fib()` with an independent iterative reference.
  - Cover every supported fixed-width type and valid/overflow boundaries.
  - Add tests for lookup-only, iterative-only, and hybrid threshold configurations.

- [ ] Benchmark the unified `FibInteger::fib()` policy by type.
  - Sweep lookup-table sizes and `SIMPLE_THRESHOLD` values rather than benchmarking only one
    maximum index per type.
  - Include lookup-only, iterative-only, lookup-plus-iteration, lookup-plus-doubling, and the
    full lookup-plus-iteration-plus-doubling configurations where they are representable.
  - Measure both cold boundary cases and representative interior indices.
  - Include arithmetic cost for `u64`, `u128`, and `U256` separately.

- [ ] Validate and document the unified policy contract.
  - Define the relationship between `lookup_size()` and the values returned by `lookup()`.
  - Define whether `SIMPLE_THRESHOLD` is inclusive.
  - Handle thresholds of `0`, `1`, and `usize::MAX` without underflow or overflow.
  - Decide how `FibFit::Unknown` is checked before fixed-width arithmetic can overflow.

## More numeric types

- [ ] Support more integer types where useful for the exercise.
  - Consider additional fixed-width integer types or fixed-size bigint types.
  - Document the fit and overflow behavior for each type.
  - Add fixed-width types `U1024`, `U2048`, and `U4096`.
  - Benchmark their arithmetic and determine suitable `n1` and `n2` thresholds.

- [ ] Support an unbounded bigint implementation from an external crate.
  - Evaluate an appropriate arbitrary-precision unsigned integer type.
  - Determine how `fits_fibonacci()` should behave for an unbounded type.
  - Add formatting and arithmetic tests specific to the external bigint.
  - Compare its behavior and performance with the fixed-width types.
