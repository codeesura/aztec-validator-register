# Aztec Validator Registration Tool

A TypeScript application that registers a validator in the Aztec network on Sepolia testnet using Flashbots bundles.

## Overview

This tool allows you to register as a validator in the Aztec network by:

1. Calculating the forwarder address for your wallet
2. Submitting a bundle transaction via Flashbots to the StakingAssetHandler contract
3. Monitoring the transaction status until it's included in a block

## Prerequisites

- [Bun](https://bun.sh/) runtime (v1.0+ recommended)
- An Ethereum private key with funds on Sepolia testnet

## Installation

### Install Bun

If you don't have Bun installed yet, you can install it by running:

**macOS, Linux, or WSL**
```bash
curl -fsSL https://bun.sh/install | bash
```

**Windows (with PowerShell)**
```powershell
powershell -c "irm bun.sh/install.ps1 | iex"
```

### Set up the project

1. Clone this repository:
   ```bash
   git clone https://github.com/codeesura/aztec-validator-register.git
   cd aztec-register
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Create a `.env` file in the root directory with your private key:
   ```
   PRIVATE_KEY=your_private_key_here
   ```
   (⚠️ Never commit your .env file or share your private key!)

## Usage

Run the application:

```bash
bun run index.ts
```

The application will:
- Calculate your forwarder address
- Listen for new blocks on Sepolia
- Submit a bundle transaction to register as a validator
- Report on the status of the transaction

## Configuration

You can modify the following constants in `index.ts`:

- `CHAIN_ID`: The chain ID (11155111 for Sepolia)
- `RPC_URL`: The RPC URL for the Ethereum node
- `GWEI`: The gas price in GWEI
- `STAKING_ASSET_HANDLER_ADDRESS`: The address of the StakingAssetHandler contract

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
