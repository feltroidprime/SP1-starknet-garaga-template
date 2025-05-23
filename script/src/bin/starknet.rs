//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can have an
//! EVM-Compatible proof generated which can be verified on-chain.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release --bin starknet -- --system groth16
//! ```

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
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

/// The arguments for the Starknet command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct StarknetArgs {
    #[arg(long, default_value = "3")]
    n: u32,
    #[arg(long, value_enum, default_value = "groth16")]
    system: ProofSystem,
}

/// Enum representing the available proof systems
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProofSystem {
    Groth16,
}

/// A fixture that can be used to test the verification of SP1 zkVM proofs inside Solidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct SP1FibonacciProofFixture {
    vkey: String,
    public_values: String,
    proof: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = StarknetArgs::parse();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the program.
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);

    println!("n: {}", args.n);
    println!("Proof System: {:?}", args.system);

    // Generate the proof based on the selected proof system.
    let proof = match args.system {
        ProofSystem::Groth16 => client.prove(&pk, &stdin).groth16().run(),
    }
    .expect("failed to generate proof");

    create_proof_fixture(&proof, &vk, args.system);
}

pub fn get_sp1_garaga_starknet_calldata(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
) -> Vec<BigUint> {
    let sp1_groth16_vk = get_sp1_vk();
    let vkey_bytes: Vec<u8> = hex::decode(&vk.bytes32()[2..]).unwrap();

    let groth16_proof =
        Groth16Proof::from_sp1(vkey_bytes, proof.public_values.to_vec(), proof.bytes());

    /*
     Note:
     You can use
         garaga::calldata::full_proof_with_hints::groth16::get_groth16_calldata_felt
     instead to output the result in Vec<Felt> type,
     for better backend integration with tools like
         https://github.com/xJonathanLEI/starkli
    */
    get_groth16_calldata(&groth16_proof, &sp1_groth16_vk, CurveID::BN254).unwrap()
}

/// Converts a Vec<BigUint> to a hexadecimal string format suitable for saving to a text file.
/// Each BigUint is converted to hexadecimal and placed on its own line.
///
/// # Arguments
/// * `calldata` - A vector of BigUint values to convert
///
/// # Returns
/// A string where each line contains a hexadecimal representation of a BigUint value
///
/// # Example
/// ```
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

/// Create a fixture for the given proof.
fn create_proof_fixture(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
    system: ProofSystem,
) {
    // Deserialize the public values.
    let bytes = proof.public_values.as_slice();

    // Create the testing fixture so we can test things end-to-end.
    let fixture = SP1FibonacciProofFixture {
        vkey: vk.bytes32().to_string(),
        public_values: format!("0x{}", hex::encode(bytes)),
        proof: format!("0x{}", hex::encode(proof.bytes())),
    };

    // The verification key is used to verify that the proof corresponds to the execution of the
    // program on the given input.
    //
    // Note that the verification key stays the same regardless of the input.
    println!("Verification Key: {}", fixture.vkey);

    // The public values are the values which are publicly committed to by the zkVM.
    //
    // If you need to expose the inputs or outputs of your program, you should commit them in
    // the public values.
    println!("Public Values: {}", fixture.public_values);

    // The proof proves to the verifier that the program was executed with some inputs that led to
    // the give public values.
    println!("Proof Bytes: {}", fixture.proof);

    // Generate Starknet calldata and save it to a text file
    let calldata = get_sp1_garaga_starknet_calldata(proof, vk);
    let calldata_len = calldata.len();
    let calldata_hex_string = biguint_vec_to_hex_string(calldata);

    println!("Generated {} calldata elements", calldata_len);

    // Save the fixture to a file.
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts/src/fixtures");
    std::fs::create_dir_all(&fixture_path).expect("failed to create fixture path");

    // Save JSON fixture
    std::fs::write(
        fixture_path.join(format!("{:?}-fixture.json", system).to_lowercase()),
        serde_json::to_string_pretty(&fixture).unwrap(),
    )
    .expect("failed to write fixture");

    // Save calldata as hexadecimal text file
    std::fs::write(
        fixture_path.join(format!("{:?}-calldata.txt", system).to_lowercase()),
        calldata_hex_string,
    )
    .expect("failed to write calldata file");

    println!("Fixtures saved to: {}", fixture_path.display());
}
