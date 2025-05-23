use snforge_std::fs::{File, FileTrait, read_txt};
use snforge_std::{ContractClassTrait, DeclareResultTrait, declare};
use sp1_app::{
    IHelloStarknetDispatcher, IHelloStarknetDispatcherTrait, IHelloStarknetSafeDispatcher,
    IHelloStarknetSafeDispatcherTrait,
};
use starknet::ContractAddress;

fn deploy_contract(name: ByteArray) -> ContractAddress {
    let contract = declare(name).unwrap().contract_class();
    let (contract_address, _) = contract.deploy(@ArrayTrait::new()).unwrap();
    contract_address
}

#[test]
#[fork(url: "https://starknet-sepolia.public.blastapi.io/rpc/v0_8", block_tag: latest)]
fn test_verify_sp1_proof() {
    let contract_address = deploy_contract("HelloStarknet");
    let dispatcher = IHelloStarknetDispatcher { contract_address };
    let file = FileTrait::new("src/fixtures/groth16-calldata.txt");
    let calldata = read_txt(@file);
    let result = dispatcher.verify_sp1_proof(calldata);
    assert(result.is_some(), 'Proof is invalid');
}
