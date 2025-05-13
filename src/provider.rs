use alloy::providers::{Provider as AlloyProvider, ProviderBuilder, WsConnect};
use eyre::{Report, Result};
use std::sync::Arc;

use super::config;

pub type ConcreteProvider = Arc<dyn AlloyProvider + Send + Sync>;

pub async fn setup() -> Result<ConcreteProvider, Report> {
    let config = config::Config::new();
    let ws = WsConnect::new(config.rpc_url.clone());
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    Ok(Arc::new(provider))
}
