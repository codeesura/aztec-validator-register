use alloy::primitives::{Address, Bytes, U256, address};
use alloy::{sol, sol_types::SolCall};
use eyre::Result;
use futures_util::StreamExt;

mod config;
mod create_transaction;
mod estimate_gas;
mod forwarder;
mod provider;

use crate::config::Config;
use crate::create_transaction::{TransactionParams, create_transaction};
use crate::forwarder::get_forwarder_address;
use estimate_gas::estimate_gas;

sol! {
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Eq)]
    function addValidator(address targetProtocol, address validatorAddress) external;
}

const STAKING_ADDRESS: Address = address!("0xF739D03e98e23A7B65940848aBA8921fF3bAc4b2"); // staking contract address

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new();
    let provider = provider::setup().await?;
    let gas_price = 1_000_000_000_000; // 1k gwei

    let sub = provider.subscribe_blocks().await?;
    let mut stream = sub.into_stream();

    println!("Awaiting block headers...");
    let proposer_address = config.validator_address;
    let forwarder_address = get_forwarder_address(proposer_address);
    println!("Forwarder address: {:?}", forwarder_address);

    let add_validator_data = addValidatorCall {
        targetProtocol: proposer_address,
        validatorAddress: forwarder_address,
    }
    .abi_encode();

    println!("Proposer address: {:?}", proposer_address);

    println!(
        "Add validator data: {:?}",
        Bytes::from(add_validator_data.clone())
    );

    let mut add_validator_params = TransactionParams {
        from: Some(config.validator_address),
        to: STAKING_ADDRESS,
        signer: config.private_key.clone(),
        input: Some(Bytes::from(add_validator_data)),
        value: U256::from(0),
        nonce: None,
        gas_price: None,
        gas_limit: None,
        chain_id: Some(provider.get_chain_id().await?),
    };

    let provider_clone = provider.clone();
    let handle = tokio::spawn(async move {
        while let Some(header) = stream.next().await {
            println!("Latest block number: {}", header.number);

            let nonce = match provider_clone
                .get_transaction_count(config.validator_address)
                .await
            {
                Ok(nonce) => nonce,
                Err(_err) => {
                    continue;
                }
            };

            add_validator_params.nonce = Some(nonce);
            add_validator_params.gas_price = Some(gas_price);

            let add_validator_gas_limit =
                match estimate_gas(&provider, add_validator_params.clone()).await {
                    Ok(gas_limit) => gas_limit, // estimate gas limit 250-270k
                    Err(_e) => {
                        continue;
                    }
                };

            add_validator_params.gas_limit = Some(add_validator_gas_limit);

            let tw_raw = match create_transaction(add_validator_params.clone()).await {
                Ok(tx) => tx,
                Err(_e) => {
                    println!("Error creating transaction: {:?}", _e);
                    continue;
                }
            };

            match provider.send_tx_envelope(tw_raw.clone()).await {
                Ok(tx_hash) => println!("Transaction sent with hash: {:?}", tx_hash),
                Err(e) => eprintln!("Error sending transaction: {:?}", e),
            }
        }
    });

    handle.await?;

    Ok(())
}
