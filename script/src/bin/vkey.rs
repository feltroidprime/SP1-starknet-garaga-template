//! # SP1 Program Verification Key Extractor
//!
//! This utility extracts the verification key for an SP1 program, which is required
//! for configuring the on-chain verification contract. The verification key uniquely
//! identifies the SP1 program and ensures that only proofs generated for this specific
//! program will be accepted by the verification contract.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --release --bin vkey
//! ```
//!
//! ## Output
//!
//! The script outputs a hexadecimal string representing the verification key:
//! ```
//! 0x00ee2a4a1c9c659ed802a544aa469136e72e1a1538af94fce56705576b48f247
//! ```
//!
//! ## Integration
//!
//! Copy this verification key and update the `SP1_PROGRAM` constant in your
//! Starknet verification contract (`contracts/src/lib.cairo`):
//!
//! ```cairo
//! const SP1_PROGRAM: u256 = 0x00ee2a4a1c9c659ed802a544aa469136e72e1a1538af94fce56705576b48f247;
//! ```
//!
//! ## Security Note
//!
//! The verification key is derived from the compiled SP1 program binary and
//! changes whenever the program logic is modified. Always regenerate and update
//! the verification key after making changes to the SP1 program.

use sp1_sdk::{include_elf, HashableKey, Prover, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
///
/// This is the same program binary used in the main execution and proving scripts.
/// The verification key is derived from this compiled program.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

/// Extract and display the verification key for the SP1 Fibonacci program.
///
/// This function:
/// 1. Creates a CPU-based prover client (faster for key generation)
/// 2. Sets up the program to generate the verification key
/// 3. Extracts the verification key in hexadecimal format
/// 4. Displays the key for use in the verification contract
///
/// ## Key Properties
///
/// The verification key:
/// - Uniquely identifies the SP1 program
/// - Is deterministic (same program = same key)
/// - Is required for on-chain proof verification
/// - Must match between proof generation and verification
fn main() {
    println!("🔑 Extracting SP1 program verification key...");

    // Create a CPU-based prover for faster key generation
    // We don't need the full proving capabilities here, just key extraction
    let prover = ProverClient::builder().cpu().build();

    // Set up the program and extract the verification key
    // This process analyzes the program binary and generates the corresponding key
    let (_, vk) = prover.setup(FIBONACCI_ELF);

    // Convert the verification key to a hexadecimal string format
    // This format is compatible with both Rust and Cairo contracts
    let vkey_hex = vk.bytes32();

    println!("✅ Verification key extracted successfully!");
    println!();
    println!("📋 Verification Key:");
    println!("{}", vkey_hex);
    println!();
    println!("📝 Next Steps:");
    println!("1. Copy the verification key above");
    println!("2. Update contracts/src/lib.cairo:");
    println!("   const SP1_PROGRAM: u256 = {};", vkey_hex);
    println!("3. Regenerate proofs if the key has changed");
    println!();
    println!("💡 Note: This key uniquely identifies your SP1 program.");
    println!("   It will change if you modify the program logic.");
}
