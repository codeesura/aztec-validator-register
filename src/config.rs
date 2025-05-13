use alloy::{primitives::Address, signers::local::PrivateKeySigner};
use std::env;

pub struct Config {
    pub rpc_url: String,
    pub private_key: PrivateKeySigner,
    pub validator_address: Address,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let private_key: PrivateKeySigner = env::var("PRIVATE_KEY")
            .expect("PRIVATE_KEY must be set")
            .parse()
            .expect("should parse private key");

        let validator_address = private_key.address();

        Config {
            rpc_url: "wss://ethereum-sepolia-rpc.publicnode.com/".to_string(),
            private_key,
            validator_address,
        }
    }
}
