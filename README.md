# Boundless Foundry Template

This template serves as a starter app powered by verifiable compute from [Boundless](https://docs.beboundless.xyz). It is built around a simple smart contract, `EvenNumber` deployed on Sepolia, and its associated RISC Zero guest, `is-even`.

## Set up your environment variables

Export your Sepolia wallet private key as an environment variable (making sure it has enough funds):

```bash
export WALLET_PRIVATE_KEY="YOUR_WALLET_PRIVATE_KEY"
```

For provers to access the zkVM guest ELF binary, it must be uploaded to IPFS. This example uses [Pinata](https://pinata.cloud/). Pinata has a free tier with plenty of quota to get started. Create an account, generate an API key, and set the JWT as an environment variable:

```bash
export PINATA_JWT="YOUR_PINATA_JWT"
```

To load the rest of the environment variables (i.e. Boundless contract deployments), run:

```bash
. ./.env
```

> If you'd like to deploy your own version of the `EvenNumber.sol` contract, please run:
> `forge script contracts/scripts/Deploy.s.sol --rpc-url ${RPC_URL:?} --broadcast -vv`
> and save the deployed contract address to the .env file.

## Run the example app

The [example app](apps/src/main.rs) will upload your zkVM guest to IPFS, submit a request to the market for a proof that "4" is an even number, wait for the request to be fulfilled, and then submit that proof to the EvenNumber contract, setting the value to "4".

To run the example:

```bash
RUST_LOG=info cargo run --bin app -- --even-number-address ${EVEN_NUMBER_ADDRESS:?} --number 4
```

## Local Development 

### Build

To build the example run:

```bash
# Install RISC Zero toolchain if not already installed
curl -L https://risczero.com/install | bash
rzup install

# Populate the `./lib` submodule dependencies
git submodule update --init --recursive
cargo build
forge build
```

### Test

Test the Solidity smart contracts with:

```bash
forge test -vvv
```

Test the Rust code including the guest with:

```bash
cargo test
```


