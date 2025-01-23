# Embedding Degree Computation in Finite Fields Using Rust

This Rust program computes the **embedding degree** `k`, given the characteristic `p` of a finite field $\mathbb{F}_p$ and the order `r` of a subgroup of the multiplicative group $\mathbb{F}_p^*$. The embedding degree `k` is the smallest positive integer such that `r` divides `p^k - 1`.

## Features

- Computes the embedding degree for any valid inputs `p` and `r`.
- Handles large integers using the `num-bigint` crate.
- Provides clear output for valid computations or a failure message if the embedding degree cannot be determined within a reasonable limit.

## Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

## Installation

1. Clone this repository: https://github.com/cypriansakwa/Embedding_Degree_Computation_in_Finite_Fields_Using_Rust.git cd Embedding_Degree_Computation_in_Finite_Fields_Using_Rust
2. Add the required dependencies to `Cargo.toml`:
   ```
   num-bigint = "0.4"
   num-integer = "0.1"
   num-traits = "0.2"
   ```
## Usage

1. Open the project directory and run the program: cargo run
2. Modify the values of `p` and `r` in the `main` function to compute the embedding degree for different inputs:
```rust
let p = 13u32.to_biguint().unwrap(); // Field characteristic
let r = 5u32.to_biguint().unwrap(); // Subgroup order
```
**Example Output**

For $p = 13$ and $r = 5$, the output will be:
```
The embedding degree is: 4
```


