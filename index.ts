import {
  StakingAssetHandlerAbi,
  ForwarderBytecode,
} from "@aztec/l1-artifacts";
import { ethers } from "ethers";
import { FlashbotsBundleProvider, FlashbotsBundleResolution } from "@flashbots/ethers-provider-bundle";
import dotenv from "dotenv";

dotenv.config();

// Configuration constants
const CHAIN_ID = 11155111; // Sepolia testnet
const RPC_URL = "https://sepolia.gateway.tenderly.co/";
const FLASHBOTS_RELAY = "https://relay-sepolia.flashbots.net";
const DEPLOYER_ADDRESS = "0x4e59b44847b379578588920cA78FbF26c0B4956C";
const STAKING_ASSET_HANDLER_ADDRESS = "0xF739D03e98e23A7B65940848aBA8921fF3bAc4b2";
const GWEI = "1000";

// Check for private key in environment variables
const PRIVATE_KEY = process.env.PRIVATE_KEY;
if (!PRIVATE_KEY) {
  throw new Error("PRIVATE_KEY not set in .env");
}

// Setup provider and wallet
const provider = new ethers.providers.JsonRpcProvider(RPC_URL, CHAIN_ID);
const wallet = new ethers.Wallet(PRIVATE_KEY, provider);
const authSigner = ethers.Wallet.createRandom();

/**
 * Calculate the forwarder address for a proposer
 * @param proposerAddress The address of the proposer
 * @returns The calculated forwarder address
 */
function getForwarderAddress(proposerAddress: string): string {
  const salt = proposerAddress;

  const abiCoder = new ethers.utils.AbiCoder();
  const constructorArgs = abiCoder.encode(["address"], [proposerAddress]);
  const encodedBytecode = ForwarderBytecode + constructorArgs.slice(2);

  const forwarderAddress = ethers.utils.getCreate2Address(
    DEPLOYER_ADDRESS,
    ethers.utils.keccak256(ethers.utils.toUtf8Bytes(salt)),
    ethers.utils.keccak256(encodedBytecode)
  );

  return forwarderAddress;
}

/**
 * Main function that handles the Flashbots bundle submission
 */
async function main() {
  try {
    // Log wallet information
    const walletAddress = wallet.address;
    console.log("Wallet address:", walletAddress);

    // Initialize Flashbots provider
    const flashbotsProvider = await FlashbotsBundleProvider.create(
      provider,
      authSigner,
      FLASHBOTS_RELAY,
      "sepolia"
    );
    
    // Get forwarder address
    const forwarderAddress = getForwarderAddress(walletAddress);
    console.log("Forwarder Address:", forwarderAddress);

    // Prepare transaction data for adding validator
    const stakingAssetHandlerInterface = new ethers.utils.Interface(
      StakingAssetHandlerAbi
    );

    const data = stakingAssetHandlerInterface.encodeFunctionData(
      "addValidator",
      [walletAddress, forwarderAddress]
    );

    // Listen for new blocks to submit bundles
    provider.on("block", async (blockNumber) => {
      console.log(`New block: ${blockNumber}, preparing bundle...`);
      
      // Create transaction bundle
      const bundle = [
        {
          transaction: {
            to: STAKING_ASSET_HANDLER_ADDRESS,
            data: data,
            chainId: CHAIN_ID,
            type: 2,
            gasLimit: ethers.BigNumber.from(2_000_000),
            maxFeePerGas: ethers.utils.parseUnits(GWEI, "gwei"),
            maxPriorityFeePerGas: ethers.utils.parseUnits(GWEI, "gwei"),
            value: ethers.constants.Zero,
          },
          signer: wallet,
        },
      ];

      // Sign bundle transactions
      const signedTransactions = await flashbotsProvider.signBundle(bundle);
      
      // Simulate transaction bundle
      const simulation = await flashbotsProvider.simulate(
        signedTransactions,
        blockNumber + 1
      );

      // Handle simulation results
      if ("error" in simulation) {
        console.log(`Simulation Error: ${simulation.error.message}`);
      } 
      
      // Submit bundle
      console.log(`Submitting bundle for block ${blockNumber + 1}...`);
      const flashbotsTransactionResponse = await flashbotsProvider.sendBundle(
        bundle,
        blockNumber + 1,
      );
      
      console.log(`Bundle submitted, waiting for inclusion...`);
      
      // Handle submission response
      if ('error' in flashbotsTransactionResponse) {
        console.log(`Error: ${flashbotsTransactionResponse.error.message}`);
      } else {
        flashbotsTransactionResponse.wait().then(resolution => {
          if (resolution === FlashbotsBundleResolution.BundleIncluded) {
            console.log(`🎉 Success! Bundle included in block ${blockNumber + 1}`);
            process.exit(0);
          } else if (resolution === FlashbotsBundleResolution.BlockPassedWithoutInclusion) {
            console.log(`❌ Not included in block ${blockNumber + 1}`);
          } else {
            console.log(`⚠️ Unknown resolution: ${resolution}`);
          }
        });
      }
    });

    console.log("Waiting for new blocks...");
  } catch (error) {
    console.error("Error in main function:", error);
    process.exit(1);
  }
}

// Execute main function
main().catch(console.error);
