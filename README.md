# SP1 Starknet Template

[![Build Status](https://github.com/your-org/sp1-starknet-template/workflows/Build%20Program/badge.svg)](https://github.com/your-org/sp1-starknet-template/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **A complete template for building zero-knowledge applications on Starknet using SP1 zkVM**

This template demonstrates how to create an end-to-end [SP1](https://github.com/succinctlabs/sp1) project that generates proofs of RISC-V program execution and verifies them on Starknet. It includes a Fibonacci computation example with complete proof generation and on-chain verification.

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   SP1 Program   │───▶│  Proof Scripts  │───▶│ Starknet Verif. │
│   (Rust/RISC-V) │    │   (Groth16)     │    │   (Cairo)       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

- **SP1 Program**: Rust program that computes Fibonacci numbers in the zkVM
- **Proof Scripts**: Generate and format proofs for Starknet verification
- **Starknet Contract**: Cairo smart contract that verifies SP1 proofs on-chain

## 🚀 Quick Start

### Prerequisites

Ensure you have the following installed:

- [Rust](https://rustup.rs/) (latest stable)
- [SP1 Toolchain](https://docs.succinct.xyz/docs/sp1/getting-started/install)
- [Starknet Toolchain](https://github.com/software-mansion/starkup) (Scarb + Starknet Foundry)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-org/sp1-starknet-template.git
   cd sp1-starknet-template
   ```

2. **Install SP1**
   ```bash
   curl -L https://sp1.succinct.xyz | bash
   sp1up
   ```

3. **Install Starknet toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.starkup.sh | sh
   ```

## 📖 Usage

### 1. Execute the Program (No Proof)

Test your SP1 program without generating a proof:

```bash
cd script
cargo run --release -- --execute --n 10
```

This computes the 10th Fibonacci number and displays the result.

### 2. Generate a Core Proof

Generate an SP1 [core proof](https://docs.succinct.xyz/docs/sp1/generating-proofs/proof-types#core-default):

```bash
cd script
cargo run --release -- --prove --n 10
```

### 3. Generate Starknet-Compatible Proof

> **⚠️ Hardware Requirements**: You need at least 16GB RAM for Groth16 proof generation. See [SP1 hardware requirements](https://docs.succinct.xyz/docs/sp1/getting-started/hardware-requirements#local-proving) for details.

Generate a Groth16 proof that can be verified on Starknet:

```bash
cd script
cargo run --release --bin starknet -- --system groth16 --n 10
```

This creates:
- `contracts/src/fixtures/groth16-fixture.json`: Complete proof data
- `contracts/src/fixtures/groth16-calldata.txt`: Formatted calldata for Starknet

### 4. Test On-Chain Verification

Run the Cairo tests to verify the proof on Starknet:

```bash
cd contracts
snforge test
```

## 🔧 Configuration

### Setting Up Your Program

1. **Get your program's verification key**:
   ```bash
   cd script
   cargo run --release --bin vkey
   ```

2. **Update the contract with your verification key**:
   Edit `contracts/src/lib.cairo` and set:
   ```rust
   const SP1_PROGRAM: u256 = 0x[YOUR_VERIFICATION_KEY_HERE];
   ```

3. **Update the SP1 Verifier class hash** (if needed):
   Check [Garaga's maintained contracts](https://garaga.gitbook.io/garaga/maintained-smart-contracts) for the latest version and update:
   ```rust
   const SP1_VERIFIER_CLASS_HASH: felt252 = 0x[LATEST_CLASS_HASH];
   ```

## 🌐 Using the Prover Network

For production use or complex computations, we recommend using the [Succinct Prover Network](https://docs.succinct.xyz/docs/network/introduction).

1. **Set up your environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your network private key
   ```

2. **Generate proofs using the network**:
   ```bash
   SP1_PROVER=network NETWORK_PRIVATE_KEY=your_key cargo run --release --bin starknet
   ```

## 📁 Project Structure

```
├── program/           # SP1 program (Rust/RISC-V)
│   └── src/main.rs   # Fibonacci computation logic
├── script/           # Proof generation scripts
│   └── src/bin/
│       ├── main.rs   # Execute and prove commands
│       ├── starknet.rs # Starknet-specific proof generation
│       └── vkey.rs   # Verification key extraction
├── contracts/        # Starknet smart contracts (Cairo)
│   ├── src/lib.cairo # Main verification contract
│   └── tests/        # Contract tests
├── lib/              # Shared library
│   └── src/lib.rs    # Common types and utilities
└── .github/workflows/ # CI/CD configuration
```

## 🧪 Example: Fibonacci Computation

This template includes a complete example that:

1. **Computes Fibonacci numbers** in the SP1 zkVM
2. **Generates a zero-knowledge proof** of the computation
3. **Verifies the proof on Starknet** using a Cairo smart contract

The program takes an input `n` and computes the `n-1`th and `n`th Fibonacci numbers, proving the computation was done correctly without revealing the intermediate steps.

## 🔍 Testing

### Local Testing

```bash
# Test SP1 program execution
cd script && cargo run --release -- --execute

# Test proof generation
cd script && cargo run --release -- --prove

# Test Starknet contract
cd contracts && snforge test
```

### CI/CD

The project includes GitHub Actions workflows for:
- **Build verification**: Ensures the SP1 program builds correctly
- **Execution testing**: Verifies program execution
- **Contract testing**: Tests Starknet contract functionality


## 📚 Resources

- [SP1 Documentation](https://docs.succinct.xyz/)
- [Starknet Documentation](https://docs.starknet.io/)
- [Garaga Documentation](https://garaga.gitbook.io/garaga/)
- [Garaga Library](https://github.com/keep-starknet-strange/garaga)
- [Cairo Book](https://book.cairo-lang.org/)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE-MIT](LICENSE-MIT) file for details.

## 🆘 Support

- **SP1 Issues**: [SP1 GitHub Issues](https://github.com/succinctlabs/sp1/issues)
- **Template Issues**: [Create an issue](https://github.com/your-org/sp1-starknet-template/issues)
- **Garaga Support**: [Garaga Support](https://garaga.gitbook.io/garaga/support)
- **Starknet Support**: [Starknet Support](https://www.starknet.io/online-communities/)

---

**Built with ❤️ using [SP1](https://github.com/succinctlabs/sp1), [Garaga](https://github.com/keep-starknet-strange/garaga) and [Starknet](https://starknet.io/)**
