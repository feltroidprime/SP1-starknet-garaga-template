use snforge_std::fs::file_operations::File;
use snforge_std::fs::read_txt;
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

// #[derive(Drop, Clone)]
// pub struct File {
//     path: ByteArray,
// }
// pub fn read_txt(file: @File) -> Array<felt252>

fn get_proof_calldata(file: @File) -> Array<felt252> {
    read_txt(file)
}

#[test]
fn test_increase_balance() {
    let contract_address = deploy_contract("HelloStarknet");

    let dispatcher = IHelloStarknetDispatcher { contract_address };

    let balance_before = dispatcher.get_balance();
    assert(balance_before == 0, 'Invalid balance');

    dispatcher.increase_balance(42);

    let balance_after = dispatcher.get_balance();
    assert(balance_after == 42, 'Invalid balance');
}