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