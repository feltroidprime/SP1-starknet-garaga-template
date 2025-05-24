# SP1 Starknet Verification Contracts

> **📖 For project overview and quick start, see the [main README](../README.md)**
> 
> **⚙️ For proof generation, see the [script documentation](../script/README.md)**

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

## 🚀 Quick Setup

> **Prerequisites**: See [main README prerequisites](../README.md#prerequisites)

```bash
# Build the contracts
scarb build

# Run tests (requires proof fixtures)
snforge test
```

For complete setup including proof generation, see the [main README workflow](../README.md#quick-test-workflow).

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

1. **Generate your verification key** (see [script documentation](../script/README.md#3-verification-key-script-vkeyrs)):
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

> **For detailed proof generation instructions, see [script/README.md](../script/README.md#2-starknet-script-starknetrs)**

```bash
cd ../script
cargo run --release --bin starknet -- --system groth16
```

This creates test fixtures in `src/fixtures/`:
- `groth16-fixture.json`: Complete proof data with metadata
- `groth16-calldata.txt`: Formatted calldata for contract calls

### Step 4: Verify Everything Works

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

### Test Requirements

Tests require valid proof fixtures. Generate them using:

```bash
cd ../script
cargo run --release --bin starknet -- --system groth16 --n 10
cd ../contracts
snforge test
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

> **For proof generation details, see [script/README.md](../script/README.md#integration)**

Use the script utilities to generate properly formatted calldata:

```rust
// In your Rust application
use script::get_sp1_garaga_starknet_calldata;

let calldata = get_sp1_garaga_starknet_calldata(&proof, &vk);
let calldata_hex = biguint_vec_to_hex_string(calldata);
```

## 🔍 Troubleshooting

### Common Issues

1. **"Wrong program" error**
   - Regenerate verification key: `cd ../script && cargo run --release --bin vkey`
   - Update contract with new key in [Step 2](#step-2-set-your-program-verification-key)
   - Regenerate proofs: `cargo run --release --bin starknet`

2. **Test failures**
   - Ensure proof fixtures exist: check `src/fixtures/` directory
   - Regenerate fixtures: `cd ../script && cargo run --release --bin starknet`
   - Verify contract configuration matches generated proofs

3. **Fork test issues**
   - Check network connectivity to Sepolia
   - Verify Garaga verifier is deployed on current Sepolia
   - Update `SP1_VERIFIER_CLASS_HASH` if needed

### Debug Commands

```bash
# Check if fixtures exist
ls -la src/fixtures/

# Verbose test output
snforge test --verbose

# Check contract compilation
scarb build
```

## 📚 Resources

- [Garaga Documentation](https://garaga.gitbook.io/garaga/)
- [Garaga SP1 Verifier Contracts](https://garaga.gitbook.io/garaga/maintained-smart-contracts)
- [Starknet Foundry](https://foundry-rs.github.io/starknet-foundry/)
- [Cairo Book](https://book.cairo-lang.org/)
- [Starknet Documentation](https://docs.starknet.io/)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE-MIT](../LICENSE-MIT) file for details.

---

**Need help?** Check the [main project README](../README.md) or [troubleshooting section](../README.md#troubleshooting).