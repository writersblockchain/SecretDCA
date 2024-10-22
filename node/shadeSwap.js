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

const shade_swap = async () => {
  let handleMsg = {
    shade_swap: {
       amount: 10000000,
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
shade_swap();