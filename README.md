Encrypted Swap MVP - Swap sUSDC for sSCRT! 

- Get testnet SCRT [here](https://docs.scrt.network/secret-network-documentation/overview-ecosystem-and-technology/secret-network-overview/testnet#set-up-with-keplr)
- Swap testnet SCRT for testnet USDC [here](https://testnet.shadeprotocol.io/)
(use password `argos` to swap for testnet USDC :D )

1. To compile the contract, `cd` into `secret-contract`, open Docker on your machine, and run:
-  `make build-mainnet-reproducible`

2. To upload the contract, `cd` into `node`, install the dependencies, and run:
- `npm upload`
(Be sure to add your contractAddress and codeHash to the env file :D )

3. To initialize sUSDC -> sSCRT strategy, `cd` into `node` and run: 
- `node initializeStrategy`

4. If you want to query a strategy by `id`, `cd` into `node` and run: 
- `node queryStrategy`

5. To create an allowance for your wallet to swap sUSDC, `cd` into `node` and run: 
- `node increaseAllowance`

6. To execute the encrypted swap, `cd` into `node` and run: 
- `node performEncryptedSwap`




