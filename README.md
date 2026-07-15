> "Probably the most overengineered Fibonacci library out there. Clinical diagnosis: an acute nerd-sniping attack, with secondary complications involving generic traits, bigint representations, benchmark harnesses, and increasingly elaborate TODO files."
>
> — ChatGPT

# Fibonacci Learning Project

This is a small Rust learning project focused on understanding how several Rust and general
programming concepts fit together in a working program.

## Learning topics

- Rust fundamentals and general programming practices
- Generics and trait-based abstraction
- Operator traits such as addition, multiplication, and subtraction
- Numeric types and overflow considerations
- Module and crate design
- Separating reusable library code from binary application code
- Error handling with `Result`
- Testing and linting
- Implementing and using fixed-width integer types

### Generic programming and numeric choices

The `FibInteger` trait demonstrates how generic programming can separate the Fibonacci algorithm
from the numeric representation. The same algorithm can be instantiated with a type chosen for the
input range being used.

That choice can provide a more efficient implementation than having only two alternatives:

- small built-in integers, which are fast but support only a limited range; or
- an unbounded bigint, which can represent much larger values but may allocate storage as the value
	grows.

For example, if an application only needs Fibonacci values through a known index, a sufficiently
wide fixed-width type such as `u128` or `U256` can avoid on-demand allocation while still supporting
that range. Generic code lets the algorithm work with this specialized type without duplicating the
Fibonacci logic for every representation. The compiler can then generate a concrete version for the
chosen type, while the trait records the operations and fit information that the algorithm needs.

This is a useful general programming lesson: an abstraction does not always mean choosing one
universal implementation. It can also make it easy to choose an implementation whose performance
and storage characteristics match a particular input range.

### Benchmarking algorithm choices

Benchmarking adds an important qualification to the usual complexity comparison. Fast doubling
uses $O(\log n)$ arithmetic steps, while simple iteration uses $O(n)$ steps, but the asymptotic
advantage does not automatically make fast doubling faster for every practical input.

The benchmarks in this project show that, for indices whose results fit in the smaller primitive
types up to and including `u64`, the simple iterative implementation is faster than or approximately
as fast as fast doubling. For these relatively small ranges, the extra multiplication and
subtraction performed by fast doubling can cost more than the additional iterations of the simple
algorithm. This is a useful reminder that algorithmic complexity describes growth with input size,
while real performance also depends on constant factors, operation costs, and the numeric type.

The result suggests a possible hybrid implementation: use precomputed small results first, use
simple iteration through a measured threshold, and use fast doubling beyond that threshold. For a
large index, the hybrid could first calculate a suitable starting pair with simple iteration and
then continue with fast doubling, rather than assuming that fast doubling must begin at `F(0)`.

## Fibonacci algorithm

The project calculates Fibonacci numbers using the **fast-doubling algorithm** rather than the
straightforward iterative or recursive approaches.

The algorithm uses these identities:

$$ F(2k) = F(k)\left(2F(k+1)-F(k)\right) $$

$$ F(2k+1) = F(k)^2 + F(k+1)^2 $$

By examining the bits of the requested index, fast doubling computes `F(n)` in $O(\log n)$
arithmetic steps while using constant additional algorithmic space.

### Deriving Fibonacci fast-doubling identities

Understanding this part allows to rederive the formulas after society collapse. Start with the
Fibonacci recurrence:

$$ F(n+2) = F(n) + F(n+1) $$

The following table shows the first few values, followed by two consecutive values whose identities
we want to extend:

<center>

| $n$ | $0$ | $1$ | $2$ | $3$ | $4$ | $5$ | $6$ | ... | $m$ | $m+1$ |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| $F(n)$ | $0$ | $1$ | $1$ | $2$ | $3$ | $5$ | $8$ | ... | $F(m)$ | $F(m+1)$ |

</center>

Applying the recurrence repeatedly gives the following expressions in terms of $F(m)$ and $F(m+1)$:

$$ \begin{aligned} F(m+2) &= F(m) + F(m+1) \\
F(m+3) &= F(m) + 2F(m+1) \\
F(m+4) &= 2F(m) + 3F(m+1) \\
F(m+5) &= 3F(m) + 5F(m+1) \\
F(m+6) &= 5F(m) + 8F(m+1) \end{aligned} $$

The coefficients are Fibonacci numbers themselves. For example, the coefficients of $F(m+5)$ are $3
= F(4)$ and $5 = F(5)$. This suggests the general identities, for $n \geq 1$:

$$ F(m+n) = F(n-1)F(m) + F(n)F(m+1) $$

$$ F(m+n+1) = F(n)F(m) + F(n+1)F(m+1) $$

These identities can be proved by induction on $n$. They hold for the initial values $n=1$, and
advancing $n$ by one applies the Fibonacci recurrence to the two expressions.

Now set $m = n = k$. The first identity becomes:

$$ F(2k) = F(k-1)F(k) + F(k)F(k+1) $$

Using $F(k-1) = F(k+1) - F(k)$, this simplifies to:

$$ \begin{aligned} F(2k) &= F(k)\left(F(k+1)-F(k)\right) + F(k)F(k+1) \\
	&= F(k)\left(2F(k+1)-F(k)\right) \end{aligned} $$

The second identity becomes:

$$ \begin{aligned} F(2k+1) &= F(k)F(k) + F(k+1)F(k+1) \\
	  &= F(k)^2 + F(k+1)^2 \end{aligned} $$

These are the fast-doubling identities used by the implementation. Instead of calculating every
Fibonacci number up to the requested index, the algorithm uses the binary digits of the index to
repeatedly derive $F(2k)$ and $F(2k+1)$ the known pair $(F(k), F(k+1))$. This is why the number of
arithmetic steps is $O(\log n)$.

## Project structure

- `src/lib.rs` contains the public Fibonacci API, shared traits, fit information, and error types.
- `src/primint.rs` contains implementations for Rust's primitive unsigned integer types.
- `src/uint.rs` contains the fixed-width `U256` implementation as an example of going beyond the
  primitive integers.
- `src/main.rs` is the binary entry point and demonstrates calling the library.

The separation between the library and binary is intentional: the Fibonacci implementation can be
reused and tested independently of the command-line demonstration.

## Running the project

Build and run the example with Cargo:

```text
cargo run
```

Run the test suite:

```text
cargo test
```

Run Clippy's additional checks:

```text
cargo clippy --all-targets --all-features
```

## Scope

This is an educational project, not a production-ready arbitrary-precision arithmetic library. The
implementation is intentionally being developed incrementally so that each design choice—such as
trait boundaries, overflow handling, and module visibility—can be examined and understood.

## AI-assisted learning approach

AI is used in this project as a **teacher and reviewer, not as an implementor**. The learner makes
the design decisions and writes the code. AI may explain concepts, review the current
implementation, point out errors or trade-offs, and suggest exercises, but it should not edit or
implement code unless explicitly requested.

The goal is to preserve the productive parts of programming: forming hypotheses, making design
choices, encountering compiler feedback, and understanding why a solution works.

### Examples of the didactic approach

The prompts used during development deliberately ask for explanations rather than automatic changes.
For example:

> "Review the Rust code below as a mentor, not just as a bug finder. My goals are to learn idiomatic
> Rust and improve my general programming skills."

This frames the AI's role as a mentor. The review should explain correctness, ownership, traits,
error handling, complexity, testing, and maintainability instead of simply replacing the code.

The project also uses focused conceptual questions, such as:

> "why limit limbs to 1_000_000_000?"

and:

> "how would we multiply if limbs were up to 2^32 (full range u32)?"

These questions encourage understanding the representation and arithmetic invariants behind a bigint
rather than copying an implementation.

The design is then refined through questions about abstractions:

> "I would like to move from num_traits to our own FibInteger trait, which I will implement for
> types that need to be supported. How do I capture that the types must implement *, + and - ?"

and about API behavior:

> "but could we go like: if fit is no - err, if fit is yes - unchecked version, if fit is maybe -
> checked version."

Finally, the collaboration explicitly establishes the boundary:

> "never implement code unless explicitly asked to. you're a teacher, not a colleague."

This keeps the learner responsible for implementation while using AI to explain Rust's type system,
compare alternatives, identify hidden assumptions, and propose exercises for further practice.
