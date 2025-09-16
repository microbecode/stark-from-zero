# Stark from zero

Create a [Stark](https://starkware.co/stark/) prover and verifier from zero, with Rust. Everything is implemented by hand - no external libraries are utilized.

The point is to create a minimal, educational version without strong security requirements or optimizations. This prover also does not offer the Zero Knowledge privacy property.

## What’s in here

- A simple trace for Fibonacci (rows = steps, columns = state values)
- Interpolation and Low Degree Extension (LDE) over a finite field
- Merkle commitment to the extended trace
- Fiat–Shamir transcript to derive verifier challenges (sample indices, FRI betas)
- A composition polynomial that encodes the AIR rule
- Random sampling over the extended domain to check constraints
- Minimal FRI folding (educational; not a full FRI verifier)

### What's NOT in here

As mentioned above, this is an educational project. This should not be used in production.

Some of the missing pieces include:
- ZK privacy implementation. This implementation doesn't provide privacy.
- Realistic sizes for data and variables: the used finite field is small, the extension field is small, the used trace is small, ...
- Performance improvements
- Security improvements (and fixing probable existing security issues)
- Proper implementations for some of the used building blocks. For example the used hash function is far from secure

## Prover → Verifier flow (short)

1. Build a small trace for Fibonacci.
2. Interpolate each column and evaluate on a larger domain (LDE).
3. Commit to the extended trace using a Merkle tree (one hash per row).
4. Construct the composition polynomial
   C(x) = f(x+2) − f(x+1) − f(x)
   from residuals over the original domain.
5. Derive sample indices (and FRI betas) via Fiat–Shamir from the Merkle root.
6. Prover returns values and Merkle proofs at sampled rows.
7. Verifier checks Merkle proofs and evaluates C at those sampled points in the LDE; non‑zero means reject.

## Composition polynomial (Fibonacci)

For Fibonacci, the AIR is f(n) = f(n−1) + f(n−2). This is packaged as one polynomial

    C(x) = f(x+2) − f(x+1) − f(x)

It vanishes on the original domain if and only if the trace satisfies the rule. In this repo:

- Prover computes residuals on the original steps and interpolates to get C.
- Verifier evaluates C at sampled points from the extended domain.

## Running

Requirements: Rust toolchain.

Run tests:

```
cargo test
```

Run the example:

```
cargo run
```

You should see:
- Printed Fibonacci trace
- LDE + Merkle commitment (root printed)
- Fiat–Shamir sample indices
- Merkle proof checks for sampled rows
- Composition polynomial evaluations at samples (should be zero)
