# Aztec Validator Registration Tool

A Rust application that registers a validator in the Aztec network on Sepolia testnet.

## Overview

This tool allows you to register as a validator in the Aztec network by:

1. Calculating the forwarder address for your wallet
2. Submitting a transaction to the Staking contract
3. Monitoring the transaction status until it's included in a block

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)
- An Ethereum private key with funds on Sepolia testnet

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/codeesura/aztec-validator-register.git
   cd aztec-validator-register
   ```

2. Create a `.env` file in the root directory with your private key:
   ```
   PRIVATE_KEY=your_private_key_here
   ```
   (⚠️ Never commit your .env file or share your private key!)

## Usage

Build and run the application:

```bash
cargo build --release
cargo run --release
```

The application will:
- Calculate your forwarder address
- Listen for new blocks on Sepolia
- Submit a transaction to register as a validator
- Report on the status of the transaction

## Configuration

The application uses the following configuration:

- RPC URL: `wss://ethereum-sepolia-rpc.publicnode.com/`
- Staking Contract Address: `0xF739D03e98e23A7B65940848aBA8921fF3bAc4b2`

You can modify these settings in the `src/config.rs` file.

### Important: Gas Price Configuration

To successfully compete with bots for validator registration, you need to adjust the gas price in `src/main.rs`:

```rust
let gas_price = 1_000_000_000_000; // 1k gwei
```

This value (currently set to 1,000 GWEI) should be adjusted based on current network conditions. Higher values give your transaction priority but cost more. Modify this value according to your needs and the current gas prices on Sepolia.

## Project Structure

- `src/main.rs`: Entry point and main application logic
- `src/config.rs`: Configuration settings
- `src/provider.rs`: Ethereum provider setup
- `src/forwarder.rs`: Calculate forwarder address
- `src/create_transaction.rs`: Transaction creation
- `src/estimate_gas.rs`: Gas estimation

## Dependencies

Main dependencies include:
- `alloy`: Ethereum utilities
- `tokio`: Async runtime
- `dotenv`: Environment variable loading
- `eyre`: Error handling
- `serde`/`serde_json`: JSON serialization/deserialization

## License

[MIT](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
