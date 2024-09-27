# Counter Example

> This example should be run against a deployment of the Boundless market.
> See the [local devnet doc][local-devnet-guide] for info on running one locally.
> Environment variables for connecting to and interacting with the network are defined in a [.env file](./.env).

## Build

To build the example run:

```bash
cargo build
forge build
```

## Deploy

To deploy the Counter contract run:

```bash
source .env
forge script contracts/scripts/Deploy.s.sol --rpc-url ${RPC_URL:?} --broadcast -vv
```

Save the `Counter` contract address to an env variable:

<!-- TODO: Update me -->
```bash
export COUNTER_ADDRESS=#COPY COUNTER ADDRESS FROM DEPLOY LOGS
```

> You can also use the following command to set the contract address if you have `jq` installed:
>
> ```bash
> export COUNTER_ADDRESS=$(jq -re '.transactions[] | select(.contractName == "Counter") | .contractAddress' ./broadcast/Deploy.s.sol/31337/run-latest.json)
> ```

## Run the example

> **Note**: This example uses IPFS to upload the ELF; We suggest using [Pinata](https://www.pinata.cloud) as the IPFS provider.

To run the example run:

```bash
PINATA_JWT=${PINATA_JWT:?} RUST_LOG=info cargo run --bin example-counter -- --counter-address ${COUNTER_ADDRESS:?}
```

<!-- TODO: Link to GH pages instead when it's available -->
[local-devnet-guide]: https://github.com/boundless-xyz/boundless/blob/main/docs/src/broker/local_devnet.md
