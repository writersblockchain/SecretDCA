import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv"
dotenv.config()

const wallet = new Wallet(process.env.TESTNET_WALLET);

//My Secret contract 
const contractAddress = process.env.SHADE_SWAP_CONTRACT;
const contractCodeHash = process.env.SHADE_SWAP_CONTRACT_HASH;

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
    url: "https://lcd.testnet.secretsaturn.net",
    wallet: wallet,
    walletAddress: wallet.address,
  });

const perform_encrypted_swap = async () => {
  let handleMsg = {
    perform_encrypted_swap: {
      strategy_id: 0
    },
  };
  console.log("swapping sUSDC for sSCRT");

  let tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      code_hash: contractCodeHash,
      msg: handleMsg,
    },
    {
      gasLimit: 1_000_000,
    }
  );
  console.log(tx);
};
perform_encrypted_swap();
