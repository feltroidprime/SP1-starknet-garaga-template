//! # SP1 Fibonacci Computation Program
//!
//! This program demonstrates a simple computation that can be proven using SP1's
//! zero-knowledge virtual machine. It computes Fibonacci numbers and commits the
//! results as public values that can be verified on-chain.
//!
//! ## Program Flow
//! 1. Read input `n` from the prover
//! 2. Compute the `n-1`th and `n`th Fibonacci numbers
//! 3. Encode the results as public values
//! 4. Commit the public values for verification
//!
//! ## Public Values
//! The program commits the following values that will be publicly verifiable:
//! - `n`: The input number
//! - `a`: The `n-1`th Fibonacci number
//! - `b`: The `n`th Fibonacci number
//!
//! ## Usage
//! This program is executed within the SP1 zkVM and generates proofs that can
//! be verified on Starknet using the corresponding verification contract.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use fibonacci_lib::{fibonacci, PublicValuesStruct};

/// Main entry point for the SP1 Fibonacci computation program.
///
/// This function demonstrates the complete flow of a zero-knowledge computation:
/// 1. Reading private inputs from the prover
/// 2. Performing the computation (Fibonacci sequence)
/// 3. Committing public outputs for verification
///
/// ## Zero-Knowledge Properties
/// - **Private Input**: The computation process and intermediate values remain private
/// - **Public Output**: Only the final results (n, a, b) are publicly committed
/// - **Verifiable**: The proof can be verified without re-executing the computation
///
/// ## Fibonacci Computation
/// For input `n`, the program computes:
/// - `a = fibonacci(n-1)` (the previous Fibonacci number)
/// - `b = fibonacci(n)` (the current Fibonacci number)
///
/// This demonstrates that the prover knows how to compute the nth Fibonacci number
/// without revealing the computation steps.
pub fn main() {
    // Step 1: Read the input from the prover
    //
    // This input is private to the prover and not revealed in the proof.
    // The prover provides this value when generating the proof, and it's
    // used as the starting point for our computation.
    let n = sp1_zkvm::io::read::<u32>();

    // Step 2: Perform the Fibonacci computation
    //
    // This uses a function from the workspace library crate to compute
    // the Fibonacci sequence. The computation happens inside the zkVM,
    // so the intermediate steps are not revealed in the final proof.
    let (a, b) = fibonacci(n);

    // Step 3: Prepare public values for commitment
    //
    // We encode the computation results in a format that can be easily
    // verified on-chain. The PublicValuesStruct ensures compatibility
    // with both Solidity and Cairo verification contracts.
    //
    // Note: Garaga expects all public inputs to be encoded in multiples
    // of 32 bytes. The SolType encoding handles this requirement automatically.
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { n, a, b });

    // Step 4: Commit the public values
    //
    // This is the crucial step that makes the computation results publicly
    // verifiable. The committed values will be included in the proof and
    // can be extracted by the verification contract on Starknet.
    //
    // These committed values prove that:
    // - The prover computed Fibonacci numbers for input `n`
    // - The results `a` and `b` are correct
    // - The computation was performed according to the program logic
    sp1_zkvm::io::commit_slice(&bytes);
}
