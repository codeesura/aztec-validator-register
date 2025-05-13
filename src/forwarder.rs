use alloy::primitives::{Address, address, hex, keccak256};
use serde_json::Value;
use std::fs;

pub fn get_forwarder_address(proposer: Address) -> Address {
    let forwarder_json = fs::read_to_string("src/contracts/Forwarder.json")
        .expect("Failed to read Forwarder.json file");

    let forwarder: Value =
        serde_json::from_str(&forwarder_json).expect("Failed to parse Forwarder.json");

    let bytecode_hex = forwarder["bytecode"]
        .as_str()
        .expect("Failed to get bytecode from Forwarder.json")
        .trim_start_matches("0x");

    let mut bytecode = hex::decode(bytecode_hex).expect("Invalid bytecode");

    let factory = address!("0x4e59b44847b379578588920cA78FbF26c0B4956C");
    let mut encoded_args = vec![0u8; 32];
    encoded_args[12..32].copy_from_slice(proposer.as_slice());

    bytecode.extend_from_slice(&encoded_args);

    let init_code_hash = keccak256(&bytecode);

    let mut salt = [0u8; 32];
    salt[12..32].copy_from_slice(proposer.as_slice());

    let mut data = vec![0xff];
    data.extend_from_slice(factory.as_slice());
    data.extend_from_slice(&salt);
    data.extend_from_slice(init_code_hash.as_slice());

    let hash = keccak256(&data);
    Address::from_slice(&hash[12..])
}
