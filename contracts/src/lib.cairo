/// Interface representing `HelloContract`.
/// This interface allows modification and retrieval of the contract balance.
#[starknet::interface]
pub trait IHelloStarknet<TContractState> {
    /// Verify a SP1 proof.
    /// Returns Option::None if proof is invalid
    /// Returns Option::Some(public_inputs) if proof is valid against the expected program.
    fn verify_sp1_proof(ref self: TContractState, proof: Array<felt252>) -> Option<Span<u256>>;
}

/// Simple contract for managing balance.
#[starknet::contract]
mod HelloStarknet {
    use starknet::SyscallResultTrait;
    use starknet::storage::{StoragePointerReadAccess, StoragePointerWriteAccess};
    use starknet::syscalls::library_call_syscall;
    #[storage]
    struct Storage {
        last_fibonacci_n: u256,
    }
    // The SP1 Verifier class hash, available on mainnet and sepolia.
    // Declared by garaga library.
    const SP1_VERIFIER_CLASS_HASH: felt252 =
        0x5d147e9fcb648e847da819287b8f462ce9416419240c64d35640dcba35e127;

    const SP1_PROGRAM: u256 = 0x00ee2a4a1c9c659ed802a544aa469136e72e1a1538af94fce56705576b48f247;

    #[abi(embed_v0)]
    impl HelloStarknetImpl of super::IHelloStarknet<ContractState> {
        fn verify_sp1_proof(ref self: ContractState, proof: Array<felt252>) -> Option<Span<u256>> {
            let mut result_serialized = library_call_syscall(
                SP1_VERIFIER_CLASS_HASH.try_into().unwrap(),
                selector!("verify_sp1_groth16_proof_bn254"),
                proof.span(),
            )
                .unwrap_syscall();

            let result = Serde::<Option<(u256, Span<u256>)>>::deserialize(ref result_serialized)
                .unwrap();

            if result.is_none() {
                return None;
            }
            let (vk, public_inputs) = result.unwrap();
            assert(vk == SP1_PROGRAM, 'Wrong program');
            Some(public_inputs)
        }
    }
}
