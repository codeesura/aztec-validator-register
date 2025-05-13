use alloy::{
    consensus::TxEnvelope,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, Bytes, U256},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[derive(Clone, Debug)]
pub struct TransactionParams {
    pub from: Option<Address>,
    pub signer: PrivateKeySigner,
    pub to: Address,
    pub input: Option<Bytes>,
    pub value: U256,
    pub nonce: Option<u64>,
    pub gas_limit: Option<u64>,
    pub gas_price: Option<u128>,
    pub chain_id: Option<u64>,
}

pub async fn create_transaction(params: TransactionParams) -> Result<TxEnvelope> {
    let mut tx = TransactionRequest::default()
        .with_to(params.to)
        .with_nonce(params.nonce.unwrap_or_default())
        .with_chain_id(params.chain_id.unwrap_or_default())
        .with_value(params.value)
        .with_gas_limit(params.gas_limit.unwrap_or_default())
        .with_gas_price(params.gas_price.unwrap_or_default())
        .with_max_fee_per_gas(params.gas_price.unwrap_or_default() * 2)
        .with_max_priority_fee_per_gas(params.gas_price.unwrap_or_default() * 2);

    if let Some(input) = params.input {
        tx = tx.with_input(input);
    }

    Ok(tx.build(&EthereumWallet::from(params.signer)).await?)
}
