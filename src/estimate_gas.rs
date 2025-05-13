use alloy::providers::Provider as AlloyProvider;
use alloy::{network::TransactionBuilder, rpc::types::TransactionRequest};
use eyre::Result;
use std::sync::Arc;

use crate::create_transaction::TransactionParams;

pub type ConcreteProvider = Arc<dyn AlloyProvider + Send + Sync>;

pub async fn estimate_gas(provider: &ConcreteProvider, params: TransactionParams) -> Result<u64> {
    let tx = TransactionRequest::default()
        .with_from(params.from.unwrap())
        .with_to(params.to)
        .with_value(params.value)
        .with_input(params.input.unwrap_or_default());

    let gas_estimate = provider.estimate_gas(tx).await?;

    Ok(gas_estimate)
}
