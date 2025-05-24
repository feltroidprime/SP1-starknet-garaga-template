# SP1 Proof Generation Scripts

> **📖 For project overview and quick start, see the [main README](../README.md)**

This directory contains the proof generation and utility scripts for the SP1 Starknet template. These scripts handle SP1 program execution, proof generation, and Starknet integration.

## 🏗️ Architecture

```
script/
├── src/bin/
│   ├── main.rs       # Core execution and proving
│   ├── starknet.rs   # Starknet-specific proof generation
│   └── vkey.rs       # Verification key extraction
├── Cargo.toml        # Dependencies and configuration
└── build.rs          # Build script for SP1 program compilation
```

## 📋 Scripts Overview

### 1. Main Script (`main.rs`)

The primary interface for SP1 program execution and core proof generation.

**Features:**
- Execute SP1 programs without proof generation (fast testing)
- Generate core proofs for verification
- Validate computation results
- Performance analysis with cycle counting

**Usage:**
```bash
# Execute without proof (development/testing)
cargo run --release -- --execute --n 10

# Generate core proof (verification)
cargo run --release -- --prove --n 10
```

### 2. Starknet Script (`starknet.rs`)

Specialized script for generating Starknet-compatible proofs using Garaga integration.

**Features:**
- Groth16 proof generation optimized for on-chain verification
- Automatic calldata formatting for Starknet contracts
- Test fixture generation for contract testing
- Garaga library integration for proof conversion

**Usage:**
```bash
# Generate Groth16 proof for Starknet
cargo run --release --bin starknet -- --system groth16 --n 10

# Using Prover Network (see main README for setup)
SP1_PROVER=network NETWORK_PRIVATE_KEY=your_key cargo run --release --bin starknet
```

**Output Files:**
- `../contracts/src/fixtures/groth16-fixture.json`: Complete proof metadata
- `../contracts/src/fixtures/groth16-calldata.txt`: Starknet contract calldata

### 3. Verification Key Script (`vkey.rs`)

Utility for extracting SP1 program verification keys.

**Features:**
- Extract verification keys from compiled SP1 programs
- Generate keys in format compatible with Cairo contracts
- Provide integration instructions

**Usage:**
```bash
cargo run --release --bin vkey
```

**Output:**
```
0x00ee2a4a1c9c659ed802a544aa469136e72e1a1538af94fce56705576b48f247
```

## 🚀 Quick Setup

> **Prerequisites**: See [main README prerequisites](../README.md#prerequisites)

```bash
# Build the scripts
cargo build --release

# Basic workflow (see main README for complete setup)
cargo run --release -- --execute --n 5        # Test execution
cargo run --release --bin vkey                 # Get verification key
# Update contract (see contracts/README.md)
cargo run --release --bin starknet -- --n 5   # Generate proof
```

## 🔧 Configuration

### Environment Variables

- `SP1_PROVER`: Set to `network` to use Succinct Prover Network
- `NETWORK_PRIVATE_KEY`: Your whitelisted private key for the prover network
- `RUST_LOG`: Set logging level (e.g., `info`, `debug`)

### Prover Network Setup

> **For setup instructions, see [main README](../README.md#using-the-prover-network)**

```bash
# Use network prover
SP1_PROVER=network NETWORK_PRIVATE_KEY=your_key cargo run --release --bin starknet
```

## 📊 Performance Considerations

### Hardware Requirements

| Operation | RAM | Time | Notes |
|-----------|-----|------|-------|
| Execution | 1GB | Seconds | Fast testing |
| Core Proof | 4GB | Minutes | Development proofs |
| Groth16 Proof | 16GB | 10-30 min | Production proofs |

> **💡 Tip**: For large computations, use the [Prover Network](../README.md#using-the-prover-network)

### Optimization Tips

1. **Use execution mode for development:**
   ```bash
   cargo run --release -- --execute --n 10
   ```

2. **Use Prover Network for large computations:**
   ```bash
   SP1_PROVER=network cargo run --release --bin starknet
   ```

3. **Monitor cycle counts for optimization:**
   - < 1K cycles: Excellent
   - < 10K cycles: Good
   - \> 10K cycles: Consider optimization

## 🧪 Testing and Validation

### Execution Testing

```bash
# Test different input sizes
cargo run --release -- --execute --n 1
cargo run --release -- --execute --n 10
cargo run --release -- --execute --n 20
```

### Proof Validation

```bash
# Generate and verify core proof
cargo run --release -- --prove --n 5

# Generate Starknet proof and test on-chain
cargo run --release --bin starknet -- --n 5
cd ../contracts && snforge test
```

### Verification Key Consistency

```bash
# Extract key
cargo run --release --bin vkey

# Verify it matches contract (see contracts/README.md for configuration)
grep "SP1_PROGRAM" ../contracts/src/lib.cairo
```

## 🔍 Debugging

### Common Issues

1. **"failed to generate proof"**
   - Check available RAM (16GB+ for Groth16)
   - Try using the [Prover Network](../README.md#using-the-prover-network)
   - Verify SP1 installation: `sp1 --version`

2. **"Wrong program" error in contract**
   - Regenerate verification key: `cargo run --release --bin vkey`
   - Update contract with new key (see [contracts/README.md](../contracts/README.md#step-2-set-your-program-verification-key))
   - Regenerate proofs

3. **Calldata format issues**
   - Ensure using latest Garaga version
   - Check fixture file format
   - Verify contract expects correct format

### Debug Commands

```bash
# Verbose logging
RUST_LOG=debug cargo run --release -- --execute --n 5

# Check SP1 installation
sp1 --version

# Verify program compilation
cargo check
```

## 📁 Generated Files

### Proof Fixtures

```
../contracts/src/fixtures/
├── groth16-fixture.json    # Complete proof metadata
└── groth16-calldata.txt    # Starknet contract calldata
```

### Build Artifacts

```
target/
├── release/
│   ├── main              # Execution and proving binary
│   ├── starknet          # Starknet proof generation
│   └── vkey              # Verification key extraction
└── debug/                # Debug builds
```

## 🔗 Integration

### With Starknet Contracts

> **For complete contract setup, see [contracts/README.md](../contracts/README.md)**

1. **Extract verification key:**
   ```bash
   cargo run --release --bin vkey
   ```

2. **Update contract:** (see [contracts/README.md](../contracts/README.md#step-2-set-your-program-verification-key))

3. **Generate proof and test:**
   ```bash
   cargo run --release --bin starknet -- --n 10
   cd ../contracts && snforge test
   ```

### With External Applications

```rust
use script::{get_sp1_garaga_starknet_calldata, biguint_vec_to_hex_string};

// Generate calldata
let calldata = get_sp1_garaga_starknet_calldata(&proof, &vk);
let hex_calldata = biguint_vec_to_hex_string(calldata);

// Use in your application
send_to_starknet_contract(hex_calldata);
```

## 📚 Dependencies

### Core Dependencies

- `sp1-sdk`: SP1 proving system
- `garaga-rs`: Starknet proof formatting
- `clap`: Command-line argument parsing
- `serde`: Serialization for fixtures

### Build Dependencies

- `sp1-build`: SP1 program compilation
- `fibonacci-lib`: Shared computation logic

## 📄 License

This project is licensed under the MIT License - see the [LICENSE-MIT](../LICENSE-MIT) file for details.

---

**Need help?** Check the [main project README](../README.md) or [troubleshooting section](../README.md#troubleshooting). 