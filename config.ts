interface Config {
  CHAIN_ID: number;
  RPC_URL: string;
  RELAYERS: string[];
  DEPLOYER_ADDRESS: string;
  STAKING_ASSET_HANDLER_ADDRESS: string;
}

export const config: Config = {
  CHAIN_ID: 1,
  RPC_URL: "https://ethereum-rpc.publicnode.com/",
  RELAYERS: [
    "https://rpc.titanbuilder.xyz",
    "https://mevshare-rpc.beaverbuild.org",
    "https://rsync-builder.xyz",
  ],
  DEPLOYER_ADDRESS: "0x4e59b44847b379578588920cA78FbF26c0B4956C",
  STAKING_ASSET_HANDLER_ADDRESS: "0xF739D03e98e23A7B65940848aBA8921fF3bAc4b2",
};
