import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv"
dotenv.config()

const wallet = new Wallet(process.env.TESTNET_WALLET);

//USDC Secret contract 
const contractAddress = process.env.USDC_CONTRACT;
const contractCodeHash = process.env.USDC_CONTRACT_HASH;

//My custom DCA Contract that executes shade swap
const ShadeSwapContract = process.env.SHADE_SWAP_CONTRACT;

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
    url: "https://lcd.testnet.secretsaturn.net",
    wallet: wallet,
    walletAddress: wallet.address,
  });

const increaseAllowance = async () => {
  let handleMsg = {
    increase_allowance: {
        spender: ShadeSwapContract,
        amount: "10000000000000000",
    },
  };
  console.log("increasing allowance on sUSDC contract");

  let tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: handleMsg,
    },
    {
      gasLimit: 100_000,
    }
  );
  console.log(tx);
};
increaseAllowance();