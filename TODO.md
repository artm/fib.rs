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

- [ ] Add fuzz tests.
  - Compare fast doubling with the simpler iterative implementation across many indices and supported types.
  - Test various combinations of integer types and indices.
  - Check useful invariants, including:
    - `F(0) = 0` and `F(1) = 1`.
    - `F(n + 2) = F(n) + F(n + 1)` where the values fit.
    - Fast-doubling results agree with the reference implementation.
    - A reported `No` fit result is not used to produce a value.

## Fit information and type abstraction

- [ ] Add a way for a type to report its maximum Fibonacci index as `Option<usize>`.
  - `Some(index)` represents a known maximum index.
  - `None` represents an unbounded type or a type whose maximum is not known statically.

- [ ] Derive `fits_fibonacci()` from the optional maximum-index information where appropriate.
  - Preserve the ability for types with more complex rules to provide their own implementation.
  - Decide how `Unknown` relates to `None` and to checked arithmetic.

## Performance

- [ ] Add a benchmarking harness.
  - Compare the simple iterative implementation with fast doubling.
  - Measure supported types separately.
  - Include larger indices where the types can represent the result.
  - Record whether arithmetic cost or algorithmic complexity dominates.

## More numeric types

- [ ] Support more integer types where useful for the exercise.
  - Consider additional fixed-width integer types or fixed-size bigint types.
  - Document the fit and overflow behavior for each type.

- [ ] Support an unbounded bigint implementation from an external crate.
  - Evaluate an appropriate arbitrary-precision unsigned integer type.
  - Determine how `fits_fibonacci()` should behave for an unbounded type.
  - Add formatting and arithmetic tests specific to the external bigint.
  - Compare its behavior and performance with the fixed-width types.
