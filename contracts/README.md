# SP1 Project Template Contracts

This is a template for writing a contract that uses verification of [SP1](https://github.com/succinctlabs/sp1) Groth16 proofs onchain using Garaga SP1 Verifier contract. 

## Requirements

- [Starknet Foundry](https://foundry-rs.github.io/starknet-foundry/getting-started/installation.html)
- [Scarb](https://docs.swmansion.com/scarb/download.html)

## Test

```sh
snforge test
```

## Development

#### Step 1: Set the `SP1_VERIFIER_CLASS_HASH` in `src/lib.cairo`

Make sure to use the latest version of the SP1 Verifier class hash maintained by the garaga library.
See [Starknet SP1 Verifier](https://garaga.gitbook.io/garaga/maintained-smart-contracts) for the latest version.

For example:

```rust
const SP1_VERIFIER_CLASS_HASH: felt252 = 0x5d147e9fcb648e847da819287b8f462ce9416419240c64d35640dcba35e127;
```

#### Step 2: Set the `SP1_PROGRAM` in `src/lib.cairo`

Find your program verification key by going into the `../script` directory and running `cargo run --release --bin vkey`, which will print an output like:

> 0x00ee2a4a1c9c659ed802a544aa469136e72e1a1538af94fce56705576b48f247

Then set the `SP1_PROGRAM` in `src/lib.cairo` to the output of that command, for example:

```rust
const SP1_PROGRAM: u256 = 0x00ee2a4a1c9c659ed802a544aa469136e72e1a1538af94fce56705576b48f247;
```

#### Step 3: Generate a proof

Run `cargo run --release --bin starknet -- --system groth16` to generate a proof.

This script generate a proof and a fixture that can be used to test the verification of SP1 proofs inside Cairo.

#### Step 4: Verify the proof

The script generate a proof using SP1 and then converts the proof to calldata for the SP1 Starknet Verifier contract.

A file is written in [`contracts/src/fixtures/groth16-calldata.txt`](src/fixtures/groth16-calldata.txt). 

Using starknet foundry, the test reads this file and calls the `verify_proof` function of the contract. The contract assert : 

- the proof is valid
- the verification key for the given Rust program matches the one in the contract

See [`contracts/src/lib.cairo`](src/lib.cairo) and [`contracts/tests/test_contract.cairo`](tests/test_contract.cairo) for the implementation.








### Integration within your application

We recommend reading the [script/src/bin/starknet.rs](../script/src/bin/starknet.rs) file to understand how the calldata is generated. 