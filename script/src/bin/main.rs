//! # SP1 Fibonacci Proof Generation and Execution
//!
//! This binary provides the main interface for executing and proving SP1 programs.
//! It demonstrates two key modes of operation:
//!
//! 1. **Execution Mode**: Run the program in the SP1 zkVM without generating a proof
//! 2. **Proving Mode**: Generate a cryptographic proof of program execution
//!
//! ## Usage
//!
//! ### Execute without proof (fast, for testing):
//! ```bash
//! cargo run --release -- --execute --n 10
//! ```
//!
//! ### Generate a core proof (slower, for verification):
//! ```bash
//! cargo run --release -- --prove --n 10
//! ```
//!
//! ## Features
//!
//! - **Configurable Input**: Specify the Fibonacci number to compute via `--n` parameter
//! - **Execution Verification**: Validates computation results against expected values
//! - **Cycle Counting**: Reports the number of execution cycles for performance analysis
//! - **Proof Generation**: Creates verifiable proofs of correct computation
//!
//! ## Zero-Knowledge Properties
//!
//! When generating proofs, this script demonstrates:
//! - **Completeness**: Valid computations always produce valid proofs
//! - **Soundness**: Invalid computations cannot produce valid proofs
//! - **Zero-Knowledge**: Proofs reveal only the public outputs, not computation steps

use alloy_sol_types::SolType;
use clap::Parser;
use fibonacci_lib::PublicValuesStruct;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
///
/// This binary contains the compiled SP1 Fibonacci program that will be executed
/// within the zero-knowledge virtual machine. The program is embedded at compile
/// time using the `include_elf!` macro.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

/// Command-line arguments for the SP1 Fibonacci demonstration.
///
/// This structure defines the available options for running the program:
/// - Execution mode vs. proving mode (mutually exclusive)
/// - Input parameter for the Fibonacci computation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Execute the program without generating a proof.
    ///
    /// This mode runs the SP1 program in the zkVM and displays the results
    /// without the computational overhead of proof generation. Useful for:
    /// - Testing program logic
    /// - Debugging computation issues
    /// - Performance analysis (cycle counting)
    #[arg(long)]
    execute: bool,

    /// Generate a cryptographic proof of program execution.
    ///
    /// This mode runs the SP1 program and generates a core proof that can
    /// be verified independently. The proof demonstrates that the computation
    /// was performed correctly without revealing intermediate steps.
    #[arg(long)]
    prove: bool,

    /// The input number for Fibonacci computation.
    ///
    /// Specifies which Fibonacci number to compute. The program will calculate
    /// both F(n-1) and F(n) and include them in the public outputs.
    ///
    /// Default: 20 (computes F(19)=4181 and F(20)=6765)
    #[arg(long, default_value = "20")]
    n: u32,
}

/// Main entry point for the SP1 Fibonacci demonstration.
///
/// This function orchestrates the entire workflow:
/// 1. Parse command-line arguments
/// 2. Set up the SP1 prover client
/// 3. Prepare program inputs
/// 4. Execute or prove the program based on the selected mode
/// 5. Validate and display results
///
/// ## Error Handling
///
/// The function will exit with an error code if:
/// - Both `--execute` and `--prove` are specified (or neither)
/// - SP1 program execution fails
/// - Proof generation fails
/// - Proof verification fails
/// - Computation results don't match expected values
fn main() {
    // Initialize logging for detailed execution information
    sp1_sdk::utils::setup_logger();

    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    // Parse and validate command-line arguments
    let args = Args::parse();

    // Ensure exactly one mode is selected
    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Initialize the SP1 prover client
    // This client handles communication with the zkVM and proof generation
    let client = ProverClient::from_env();

    // Prepare program inputs
    // The SP1 program expects a single u32 input representing the Fibonacci index
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);

    println!("n: {}", args.n);

    if args.execute {
        // Execute the program
        println!("🚀 Executing SP1 program...");
        let (output, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();
        println!("✅ Program executed successfully.");

        // Read the output.
        let decoded = PublicValuesStruct::abi_decode(output.as_slice()).unwrap();
        let PublicValuesStruct { n: result_n, a, b } = decoded;

        // Display the computation results
        println!("📊 Computation Results:");
        println!("  Input (n): {}", result_n);
        println!("  F(n-1): {}", a);
        println!("  F(n): {}", b);

        // Validate the results against expected values
        let (expected_a, expected_b) = fibonacci_lib::fibonacci(args.n);
        assert_eq!(
            a, expected_a,
            "F(n-1) mismatch: expected {}, got {}",
            expected_a, a
        );
        assert_eq!(
            b, expected_b,
            "F(n) mismatch: expected {}, got {}",
            expected_b, b
        );
        assert_eq!(
            result_n, args.n,
            "Input mismatch: expected {}, got {}",
            args.n, result_n
        );

        println!("✅ Values are correct!");

        // Report execution statistics
        println!("📈 Execution Statistics:");
        println!("  Total cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        println!("🔧 Setting up proving system...");
        let (pk, vk) = client.setup(FIBONACCI_ELF);
        println!("✅ Setup complete.");

        // Generate the proof
        println!("🔐 Generating proof...");
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("✅ Successfully generated proof!");

        // Verify the proof.
        println!("🔍 Verifying proof...");
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("✅ Successfully verified proof!");

        // Note about proof types
        println!("💡 Note: This is a 'core' proof suitable for development.");
        println!("   For on-chain verification, use the Starknet-specific script:");
        println!("   cargo run --release --bin starknet -- --system groth16");
    }
}
