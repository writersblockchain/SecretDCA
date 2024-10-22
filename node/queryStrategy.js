import { SecretNetworkClient } from "secretjs"
import dotenv from "dotenv"
dotenv.config()

let query = async () => {
  const secretjs = new SecretNetworkClient({
    url: "https://lcd.testnet.secretsaturn.net",
    chainId: "pulsar-3",
  })

//My Secret contract 
const contractAddress = process.env.SHADE_SWAP_CONTRACT;
const contractCodeHash = process.env.SHADE_SWAP_CONTRACT_HASH;

const query_tx = await secretjs.query.compute.queryContract({
    contract_address: contractAddress,
    code_hash:contractCodeHash,
    query: { query_strategy: {
      id: 0,
      
    } },
  })
  console.log(query_tx)
}

query()