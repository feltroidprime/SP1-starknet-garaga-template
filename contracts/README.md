# SP1 Starknet Verification Contracts

This directory contains Cairo smart contracts for verifying [SP1](https://github.com/succinctlabs/sp1) zero-knowledge proofs on Starknet using the Garaga SP1 Verifier.

## 🏗️ Architecture

The verification system consists of:

- **SP1 Verifier Contract**: Deployed by Garaga library (external dependency)
- **HelloStarknet Contract**: Your application contract that verifies SP1 proofs
- **Test Suite**: Comprehensive tests using real proof fixtures

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   SP1 Proof     │───▶│ HelloStarknet   │───▶│ Garaga Verifier │
│   (Groth16)     │    │   Contract      │    │   (Library)     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🚀 Quick Start

### Prerequisites

- [Starknet Foundry](https://foundry-rs.github.io/starknet-foundry/getting-started/installation.html)
- [Scarb](https://docs.swmansion.com/scarb/download.html)

### Installation

```bash
# Install Scarb (Cairo package manager)
curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh

# Install Starknet Foundry (testing framework)
curl -L https://raw.githubusercontent.com/foundry-rs/starknet-foundry/master/scripts/install.sh | sh
```

### Build and Test

```bash
# Build the contracts
scarb build

# Run all tests
snforge test
```

## 📁 Project Structure

```
contracts/
├── src/
│   ├── lib.cairo              # Main verification contract
│   └── fixtures/              # Test fixtures and proof data
│       ├── groth16-fixture.json    # Complete proof fixture
│       └── groth16-calldata.txt    # Formatted calldata
├── tests/
│   └── test_contract.cairo    # Contract verification tests
├── Scarb.toml                 # Cairo project configuration
└── snfoundry.toml            # Starknet Foundry configuration
```

## 🔧 Configuration

### Step 1: Update SP1 Verifier Class Hash

The contract uses the Garaga-maintained SP1 Verifier. Check [Starknet SP1 Verifier](https://garaga.gitbook.io/garaga/maintained-smart-contracts) for the latest version.

Update in `src/lib.cairo`:
```rust
const SP1_VERIFIER_CLASS_HASH: felt252 = 0x5d147e9fcb648e847da819287b8f462ce9416419240c64d35640dcba35e127;
```

### Step 2: Set Your Program Verification Key

1. **Generate your verification key**:
   ```bash
   cd ../script
   cargo run --release --bin vkey
   ```
   
   Output example: `0x00ee2a4a1c9c659ed802a544aa469136e72e1a1538af94fce56705576b48f247`

2. **Update the contract**:
   ```rust
   const SP1_PROGRAM: u256 = 0x00ee2a4a1c9c659ed802a544aa469136e72e1a1538af94fce56705576b48f247;
   ```

### Step 3: Generate Proof Fixtures

Generate proof data for testing:

```bash
cd ../script
cargo run --release --bin starknet -- --system groth16
```

This creates test fixtures in `src/fixtures/`:
- `groth16-fixture.json`: Complete proof data with metadata
- `groth16-calldata.txt`: Formatted calldata for contract calls

### Step 4: Verify Everything Works

Run the verification test:

```bash
snforge test
```

## 🧪 Contract Interface

### `IHelloStarknet`

The main contract interface for SP1 proof verification:

```rust
#[starknet::interface]
pub trait IHelloStarknet<TContractState> {
    /// Verify an SP1 proof against the expected program
    /// 
    /// # Arguments
    /// * `proof` - Array of felt252 values representing the Groth16 proof
    /// 
    /// # Returns
    /// * `Option<Span<u256>>` - Some(public_inputs) if valid, None if invalid
    fn verify_sp1_proof(ref self: TContractState, proof: Array<felt252>) -> Option<Span<u256>>;
}
```

### Verification Process

The contract performs the following verification steps:

1. **Proof Verification**: Calls the Garaga SP1 Verifier to validate the Groth16 proof
2. **Program Verification**: Ensures the proof corresponds to the expected SP1 program
3. **Public Input Extraction**: Returns the public inputs if verification succeeds

```rust
fn verify_sp1_proof(ref self: ContractState, proof: Array<felt252>) -> Option<Span<u256>> {
    // Call Garaga verifier
    let result = library_call_syscall(
        SP1_VERIFIER_CLASS_HASH.try_into().unwrap(),
        selector!("verify_sp1_groth16_proof_bn254"),
        proof.span(),
    ).unwrap_syscall();

    // Deserialize result
    let result = Serde::<Option<(u256, Span<u256>)>>::deserialize(ref result_serialized).unwrap();

    if result.is_none() {
        return None;
    }
    
    let (vk, public_inputs) = result.unwrap();
    
    // Verify this proof is for our expected program
    assert(vk == SP1_PROGRAM, 'Wrong program');
    
    Some(public_inputs)
}
```

## 🧪 Testing

### Test Structure

The test suite uses Starknet Foundry with fork testing to verify proofs against the actual Garaga verifier on Sepolia testnet.

```rust
#[test]
#[fork(url: "https://starknet-sepolia.public.blastapi.io/rpc/v0_8", block_tag: latest)]
fn test_verify_sp1_proof() {
    // Deploy contract
    let contract_address = deploy_contract("HelloStarknet");
    let dispatcher = IHelloStarknetDispatcher { contract_address };
    
    // Load proof fixture
    let file = FileTrait::new("src/fixtures/groth16-calldata.txt");
    let calldata = read_txt(@file);
    
    // Verify proof
    let result = dispatcher.verify_sp1_proof(calldata);
    assert(result.is_some(), 'Proof is invalid');
}
```

### Running Tests

```bash
# Run all tests
snforge test

# Run tests with gas reporting
snforge test --detailed-resources
```


## 🔗 Integration Guide

### Using in Your Application

1. **Import the interface**:
   ```rust
   use your_contract::{IHelloStarknetDispatcher, IHelloStarknetDispatcherTrait};
   ```

2. **Call verification**:
   ```rust
   let dispatcher = IHelloStarknetDispatcher { contract_address };
   let result = dispatcher.verify_sp1_proof(proof_calldata);
   
   match result {
       Option::Some(public_inputs) => {
           // Proof is valid, use public_inputs
           handle_valid_proof(public_inputs);
       },
       Option::None => {
           // Proof is invalid
           handle_invalid_proof();
       }
   }
   ```

### Calldata Generation

Use the script utilities to generate properly formatted calldata:

```rust
// In your Rust application
use script::get_sp1_garaga_starknet_calldata;

let calldata = get_sp1_garaga_starknet_calldata(&proof, &vk);
let calldata_hex = biguint_vec_to_hex_string(calldata);
```

## 📚 Resources

- [SP1 Documentation](https://docs.succinct.xyz/)
- [Garaga Library](https://garaga.gitbook.io/garaga/)
- [Starknet Foundry](https://foundry-rs.github.io/starknet-foundry/)
- [Cairo Book](https://book.cairo-lang.org/)
- [Starknet Documentation](https://docs.starknet.io/)



**Need help?** Check the [main project README](../README.md) or create an issue.