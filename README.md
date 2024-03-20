# stark-from-zero

Create a Stark prover and verifier from zero, with Rust. Hopefully without external libraries.

The point is to create a minimal version without strong security requirements or optimizations.

## Needed components

### Hashing

Implement a very simple hashing algorithm.

### Finite field arithmetic

Implement at least addition, subtraction and multiplication. Possibly division.

### FRI

Implement FRI logic. This requires at least:

#### Polynomial operations

Implement evaluating polynomials.

#### Merkle trees

Implement.

#### Interpolation

Implement polynomial interpolation.

### Extension field arithmetic (optional)

Possibly we can get by without an extension field, at least for the first version.

### Commitment schemes (optional)

Unsure if this is strictly necessary, but probably is.
