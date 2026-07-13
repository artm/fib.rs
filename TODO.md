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

- [ ] Use the benchmark results to design a generic algorithm selector.
  - Determine how a type can communicate which Fibonacci algorithm is most efficient for a given
    output range.
  - Choose between simple iteration and fast doubling without duplicating the public API.
  - Validate that the selector is based on measurements rather than assumptions about complexity
    alone.

- [ ] Implement and benchmark a hybrid Fibonacci algorithm.
  - Define a customizable three-stage policy:
    1. Use `lookup` for indices through `n1`.
    2. Use simple iteration for indices through `n2`.
    3. Use fast doubling for larger indices.
  - Support lookup tables built at compile time or initialized lazily at startup.
  - Keep lookup optional so the implementation remains usable in hypothetical memory-constrained
    environments.
  - Allow the thresholds and lookup strategy to vary by numeric type and bigint representation.
  - For a large index, start from a suitable pair produced by lookup or simple iteration before
    continuing with fast doubling.
  - Compare the customizable hybrid against both existing implementations for every supported type.

- [ ] Design a `LookUp`/`lookup` abstraction for precomputed Fibonacci values.
  - Keep the currently measured lookup tables small because the memory cost is negligible on modern
    systems for the supported fixed-width types.
  - Preserve the ability to disable or reduce lookup tables when memory is restricted.
  - Decide whether each type provides compile-time tables, lazy startup tables, or no table.
  - Document how lookup storage and thresholds interact with dynamically growing bigints.
  - Compare the hybrid against both existing implementations for every supported numeric type.

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
