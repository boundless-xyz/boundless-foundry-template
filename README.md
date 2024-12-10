# Boundless Foundry Template

This template serves as a starting point for developing an application with verifiable compute provided by [Boundless][boundless-homepage].
It is built around a simple smart contract, `EvenNumber`, and its associated RISC Zero guest, `is-even`.

## Build

To build the example run:

```bash
# Populate the `./lib` submodule dependencies
git submodule update --init --recursive
forge build
cargo build
```

## Test

Test the Solidity smart contracts with:

```bash
forge test -vvv
```

Test the Rust code including the guest with:

```bash
cargo test
```

## Deploy

### Set up your environment

Export the following env variables with your test wallet private key and preferred RPC provider.

```bash
export RPC_URL="https://ethereum-sepolia-rpc.publicnode.com"
export WALLET_PRIVATE_KEY="YOUR_WALLET_PRIVATE_KEY"
```

A [`.env`](./.env) file is provided with the Boundless contract deployment information for Sepolia.
The example app reads from this `.env` file automatically.

### Deploy the contract on Sepolia

To deploy the `EvenNumber` contract run:

```bash
forge script contracts/scripts/Deploy.s.sol --rpc-url ${RPC_URL:?} --broadcast -vv
```

Save the `EvenNumber` contract address to an env variable:

<!-- TODO: Update me -->
```bash
# First contract deployed and top of logs is EvenNumber
export EVEN_NUMBER_ADDRESS=#COPY EVEN NUMBER ADDRESS FROM DEPLOY LOGS
```

> You can also use the following command to set the contract address if you have [`jq`][jq] installed:
>
> ```bash
> export EVEN_NUMBER_ADDRESS=$(jq -re '.transactions[] | select(.contractName == "EvenNumber") | .contractAddress' ./broadcast/Deploy.s.sol/11155111/run-latest.json)
> ```

### Run the example on Sepolia

The example app uploads the zkVM guest ELF binary and input to a public URL using a storage provider.
IPFS pinning via [Pinata](https://pinata.cloud/) is a supported and easy to set up option.
You can sign up with their free tier, which will have plenty of quota to get started.
You can also send inputs directly in your transaction, and can host your guest on any public HTTP service.

```bash
export PINATA_JWT="YOUR_PINATA_JWT"
```

To run the example run:

```bash
RUST_LOG=info cargo run --bin app -- --even-number-address ${EVEN_NUMBER_ADDRESS:?} --number 4
```

[jq]: https://jqlang.github.io/jq/
[boundless-homepage]: https://beboundless.xyz
[sepolia]: https://ethereum.org/en/developers/docs/networks/#sepolia
