//! # SP1 Starknet Proof Generation
//!
//! This script generates SP1 proofs in formats compatible with Starknet verification.
//! It creates Groth16 proofs that can be verified on-chain using the Garaga SP1 Verifier
//! and generates properly formatted calldata for Starknet contract interactions.
//!
//! ## Features
//!
//! - **Groth16 Proof Generation**: Creates zero-knowledge proofs suitable for on-chain verification
//! - **Starknet Calldata Formatting**: Converts proofs to Starknet-compatible format
//! - **Test Fixture Creation**: Generates files for testing contract verification
//! - **Garaga Integration**: Uses Garaga library for Starknet-specific proof formatting
//!
//! ## Usage
//!
//! ### Generate Groth16 proof for Starknet:
//! ```bash
//! cargo run --release --bin starknet -- --system groth16 --n 10
//! ```
//!
//! ### Using the Prover Network:
//! ```bash
//! SP1_PROVER=network NETWORK_PRIVATE_KEY=your_key cargo run --release --bin starknet
//! ```
//!
//! ## Output Files
//!
//! The script generates test fixtures in `../contracts/src/fixtures/`:
//! - `groth16-fixture.json`: Complete proof data with metadata
//! - `groth16-calldata.txt`: Formatted calldata for Starknet contract calls
//!
//! ## Hardware Requirements
//!
//! - **Minimum RAM**: 16GB for Groth16 proof generation
//! - **Recommended**: Use the Succinct Prover Network for production workloads
//!
//! ## Integration
//!
//! The generated calldata can be used directly with the Starknet verification contract
//! to test and verify SP1 proofs on-chain.

use clap::{Parser, ValueEnum};
use garaga_rs::calldata::full_proof_with_hints::groth16::{
    get_groth16_calldata, get_sp1_vk, Groth16Proof,
};
use garaga_rs::definitions::CurveID;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use sp1_sdk::{
    include_elf, HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey,
};
use std::path::PathBuf;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
///
/// This is the compiled SP1 Fibonacci program that will be proven and verified.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

/// Command-line arguments for Starknet proof generation.
///
/// This structure defines the available options for generating Starknet-compatible proofs.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct StarknetArgs {
    /// The input number for Fibonacci computation.
    ///
    /// Specifies which Fibonacci number to compute and prove.
    /// The program will calculate F(n-1) and F(n) and include them in the proof.
    ///
    /// Default: 3 (computes F(2)=1 and F(3)=2)
    #[arg(long, default_value = "3")]
    n: u32,

    /// The proof system to use for generating the proof.
    ///
    /// Currently supports Groth16, which is optimized for on-chain verification
    /// due to its constant proof size and fast verification time.
    #[arg(long, value_enum, default_value = "groth16")]
    system: ProofSystem,
}

/// Supported proof systems for Starknet verification.
///
/// Each proof system has different characteristics:
/// - **Groth16**: Constant-size proofs, fast verification, requires trusted setup
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProofSystem {
    /// Groth16 zero-knowledge proof system.
    ///
    /// Groth16 is currently the most practical choice for on-chain verification because:
    /// - Constant proof size (~200 bytes)
    /// - Fast verification time
    /// - Well-supported by Garaga library
    /// - Efficient gas costs on Starknet
    Groth16,
}

/// Test fixture containing SP1 proof data for contract testing.
///
/// This structure contains all the necessary data to test SP1 proof verification
/// in Cairo contracts. It includes both the raw proof data and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct SP1FibonacciProofFixture {
    /// The verification key for the SP1 program (hex string).
    ///
    /// This key uniquely identifies the SP1 program and must match
    /// the `SP1_PROGRAM` constant in the verification contract.
    vkey: String,

    /// The public values committed by the SP1 program (hex string).
    ///
    /// These are the publicly verifiable outputs of the computation,
    /// encoded in a format suitable for on-chain verification.
    public_values: String,

    /// The complete proof data (hex string).
    ///
    /// This contains the cryptographic proof that can be verified
    /// to confirm the computation was performed correctly.
    proof: String,
}

/// Main entry point for Starknet proof generation.
///
/// This function orchestrates the complete workflow:
/// 1. Parse command-line arguments
/// 2. Set up the SP1 prover client
/// 3. Generate the specified type of proof
/// 4. Create test fixtures for contract verification
///
/// ## Process Flow
///
/// 1. **Setup**: Initialize prover and prepare program inputs
/// 2. **Proving**: Generate the cryptographic proof using SP1
/// 3. **Formatting**: Convert proof to Starknet-compatible format using Garaga
/// 4. **Output**: Save fixtures for testing and integration
fn main() {
    // Initialize logging for detailed execution information
    sp1_sdk::utils::setup_logger();

    // Parse command-line arguments
    let args = StarknetArgs::parse();

    // Initialize the SP1 prover client
    // This client handles communication with the zkVM and proof generation
    let client = ProverClient::from_env();

    // Set up the program for proving
    // This generates the proving and verification keys for the SP1 program
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    // Prepare program inputs
    // The SP1 program expects a single u32 input representing the Fibonacci index
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);

    println!("🔢 Input (n): {}", args.n);
    println!("🔧 Proof System: {:?}", args.system);
    println!("🚀 Generating proof...");

    // Generate the proof based on the selected proof system
    let proof = match args.system {
        ProofSystem::Groth16 => {
            println!("   Using Groth16 proof system for Starknet compatibility");
            client.prove(&pk, &stdin).groth16().run()
        }
    }
    .expect("failed to generate proof");

    println!("✅ Proof generated successfully!");

    // Create test fixtures and calldata for Starknet integration
    create_proof_fixture(&proof, &vk, args.system);
}

/// Convert SP1 proof to Starknet-compatible calldata using Garaga.
///
/// This function transforms an SP1 proof into the format expected by the
/// Garaga SP1 Verifier on Starknet. The conversion process:
///
/// 1. Extracts the verification key from the SP1 proof
/// 2. Creates a Garaga-compatible Groth16 proof structure
/// 3. Generates calldata formatted for Starknet contract calls
///
/// ## Parameters
///
/// - `proof`: The SP1 proof with public values
/// - `vk`: The SP1 verification key
///
/// ## Returns
///
/// A vector of `BigUint` values representing the calldata that can be
/// passed to the Starknet verification contract.
///
/// ## Garaga Integration
///
/// This function uses the Garaga library to:
/// - Convert SP1 proof format to Groth16 format
/// - Generate BN254 curve-compatible calldata
/// - Ensure compatibility with the on-chain verifier
pub fn get_sp1_garaga_starknet_calldata(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
) -> Vec<BigUint> {
    // Get the SP1 Groth16 verification key from Garaga
    // This is the universal verification key for SP1 Groth16 proofs
    let sp1_groth16_vk = get_sp1_vk();

    // Extract the program verification key as bytes
    // This identifies the specific SP1 program being proven
    let vkey_bytes: Vec<u8> = hex::decode(&vk.bytes32()[2..]).unwrap();

    // Create a Garaga-compatible Groth16 proof from the SP1 proof
    // This conversion handles the format differences between SP1 and Garaga
    let groth16_proof =
        Groth16Proof::from_sp1(vkey_bytes, proof.public_values.to_vec(), proof.bytes());

    // Generate Starknet calldata for the proof
    // This creates the properly formatted data for contract calls
    /*
     Note: You can use garaga::calldata::full_proof_with_hints::groth16::get_groth16_calldata_felt
     instead to output the result in Vec<Felt> type, for better backend integration with tools like
     https://github.com/xJonathanLEI/starkli
    */
    get_groth16_calldata(&groth16_proof, &sp1_groth16_vk, CurveID::BN254).unwrap()
}

/// Convert a vector of BigUint values to hexadecimal string format.
///
/// This function formats calldata for easy storage and loading in test fixtures.
/// Each BigUint value is converted to a hexadecimal string and placed on its own line.
///
/// ## Parameters
///
/// - `calldata`: A vector of BigUint values representing the proof calldata
///
/// ## Returns
///
/// A string where each line contains a hexadecimal representation of a BigUint value,
/// suitable for saving to a text file and loading in Cairo tests.
///
/// ## Format
///
/// The output format is compatible with Starknet Foundry's `read_txt` function:
/// ```
/// 0xff
/// 0xfff
/// 0x1234
/// ```
///
/// ## Example
///
/// ```rust
/// let calldata = vec![BigUint::from(255u32), BigUint::from(4095u32)];
/// let hex_string = biguint_vec_to_hex_string(&calldata);
/// // Result: "0xff\n0xfff\n"
/// ```
pub fn biguint_vec_to_hex_string(calldata: Vec<BigUint>) -> String {
    calldata
        .iter()
        .map(|big_uint| format!("0x{:x}", big_uint))
        .collect::<Vec<String>>()
        .join("\n")
        .to_string()
        + "\n" // Add final newline for proper file formatting
}

/// Create comprehensive test fixtures for the generated proof.
///
/// This function generates all the necessary files for testing SP1 proof verification
/// in Cairo contracts. It creates both human-readable JSON fixtures and
/// machine-readable calldata files.
///
/// ## Generated Files
///
/// 1. **JSON Fixture** (`{system}-fixture.json`):
///    - Complete proof metadata
///    - Verification key
///    - Public values
///    - Raw proof data
///
/// 2. **Calldata File** (`{system}-calldata.txt`):
///    - Formatted calldata for Starknet contracts
///    - Compatible with Starknet Foundry test framework
///    - Ready for direct use in contract calls
///
/// ## Parameters
///
/// - `proof`: The generated SP1 proof with public values
/// - `vk`: The SP1 verification key
/// - `system`: The proof system used (affects file naming)
///
/// ## Output Location
///
/// Files are saved to `../contracts/src/fixtures/` relative to the script directory.
fn create_proof_fixture(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
    system: ProofSystem,
) {
    println!("📁 Creating test fixtures...");

    // Extract the public values from the proof
    let bytes = proof.public_values.as_slice();

    // Create a comprehensive test fixture with all proof data
    let fixture = SP1FibonacciProofFixture {
        vkey: vk.bytes32().to_string(),
        public_values: format!("0x{}", hex::encode(bytes)),
        proof: format!("0x{}", hex::encode(proof.bytes())),
    };

    // Display key information about the proof
    println!("📋 Proof Information:");
    println!("  Verification Key: {}", fixture.vkey);
    println!("  Public Values: {}", fixture.public_values);
    println!("  Proof Size: {} bytes", proof.bytes().len());

    // Generate Starknet-compatible calldata using Garaga
    println!("🔄 Converting to Starknet calldata...");
    let calldata = get_sp1_garaga_starknet_calldata(proof, vk);
    let calldata_len = calldata.len();
    let calldata_hex_string = biguint_vec_to_hex_string(calldata);

    println!("✅ Generated {} calldata elements", calldata_len);

    // Determine the output directory for fixtures
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts/src/fixtures");
    std::fs::create_dir_all(&fixture_path).expect("failed to create fixture path");

    // Save the JSON fixture with complete proof metadata
    let json_filename = format!("{:?}-fixture.json", system).to_lowercase();
    std::fs::write(
        fixture_path.join(&json_filename),
        serde_json::to_string_pretty(&fixture).unwrap(),
    )
    .expect("failed to write JSON fixture");

    // Save the calldata as a text file for easy loading in tests
    let calldata_filename = format!("{:?}-calldata.txt", system).to_lowercase();
    std::fs::write(fixture_path.join(&calldata_filename), calldata_hex_string)
        .expect("failed to write calldata file");

    println!("💾 Fixtures saved to: {}", fixture_path.display());
    println!("   📄 {}", json_filename);
    println!("   📄 {}", calldata_filename);

    println!();
    println!("🎯 Next Steps:");
    println!("1. Run contract tests: cd ../contracts && snforge test");
    println!("2. Verify the proof on-chain using the generated calldata");
    println!("3. Integrate the verification into your application");

    println!();
    println!("💡 Integration Tips:");
    println!("- Use the JSON fixture for comprehensive testing");
    println!("- Use the calldata file for direct contract interactions");
    println!("- Ensure your contract's SP1_PROGRAM matches the verification key");
}
