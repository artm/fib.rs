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
