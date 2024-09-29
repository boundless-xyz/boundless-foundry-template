# Boundless Foundry Template

This template serves as a starting point for developing an application with verifiable compute provided by [Boundless][boundless-homepage].
It is built around a simple smart contract, `EvenNumber`, and its associated RISC Zero guest, `is-even`.

## Build

To build the example run:

```bash
cargo build
# Populate the `./lib` submodule dependancies
git submodule update --init --recursive
forge build
```

## Test

Test the Solidity smart contracts with:

```
forge test -vvv
```

Test the Rust code including the guest with:

```
cargo test
```

## Deploy

To deploy the `EvenNumber` contract run:

```bash
source .env
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
> export EVEN_NUMBER_ADDRESS=$(jq -re '.transactions[] | select(.contractName == "EvenNumber") | .contractAddress' ./broadcast/Deploy.s.sol/31337/run-latest.json)
> ```

## Run the example

> This example must be run against a deployment of the Boundless market.
> See the [local devnet doc][local-devnet-guide] for info on running one locally.
> Environment variables for connecting to and interacting with the network are defined in a [.env file](./.env).

To run the example run:

```bash
RISC0_DEV_MODE=1 RUST_LOG=info cargo run --bin app -- --even-number-address ${EVEN_NUMBER_ADDRESS:?} --number 4
```

<!-- TODO: Update link once docs are public -->
[local-devnet-guide]: https://silver-guacamole-kgzmnmn.pages.github.io/broker/local_devnet.html
[jq]: https://jqlang.github.io/jq/
[boundless-homepage]: https://beboundless.xyz
