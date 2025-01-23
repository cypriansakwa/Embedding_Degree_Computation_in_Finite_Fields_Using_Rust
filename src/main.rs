use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero}; // Import the Zero trait

fn compute_embedding_degree(p: BigUint, r: BigUint) -> Option<u32> {
    let mut k = 1u32;
    let mut power = BigUint::one();

    // Iterate until we find the smallest k such that r divides (p^k - 1)
    loop {
        power = power * &p; // Compute p^k
        let candidate = &power - BigUint::one(); // p^k - 1

        if &candidate % &r == BigUint::zero() {
            return Some(k);
        }

        k += 1;

        // Break if k is too large (to avoid infinite loops in edge cases)
        if k > 100000 {
            return None; // No embedding degree found within the limit
        }
    }
}

fn main() {
    // Example inputs
    let p = 13u32.to_biguint().unwrap(); // Characteristic of the field
    let r = 5u32.to_biguint().unwrap(); // Order of the subgroup

    match compute_embedding_degree(p, r) {
        Some(k) => println!("The embedding degree is: {}", k),
        None => println!("Failed to compute the embedding degree."),
    }
}
