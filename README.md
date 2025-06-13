# Boundless Foundry Template

This template serves as a starter app powered by verifiable compute via [Boundless](https://docs.beboundless.xyz). 

It is built around a simple smart contract, `EvenNumber` deployed on Sepolia, and its associated RISC Zero guest, `is-even`. To get you started, we have deployed to [EvenNumber contract](https://sepolia.etherscan.io/address/0xE819474E78ad6e1C720a21250b9986e1f6A866A3#code) to Sepolia; we have also pre-uploaded the `is-even` guest to IPFS.

## Set up your environment variables

Export your Sepolia wallet private key as an environment variable (making sure it has enough funds):

```bash
export PRIVATE_KEY="YOUR_PRIVATE_KEY"
```

To load the rest of the environment variables (i.e. Boundless contract deployments), run:

```bash
source .env.testnet
```

> If you'd like to deploy your own version of the `EvenNumber.sol` contract, please run:
> `forge script contracts/scripts/Deploy.s.sol --rpc-url ${RPC_URL:?} --broadcast -vv`
> and save the deployed contract address to the .env file.

## Run the example app

The [example app](apps/src/main.rs) will submit a request to the market for a proof that "4" is an even number, wait for the request to be fulfilled, and then submit that proof to the EvenNumber contract, setting the value to "4".

To run the example using the pre-uploaded zkVM guest:

```bash
RUST_LOG=info cargo run --bin app -- --rpc-url ${RPC_URL:?} --private-key ${PRIVATE_KEY:?} --even-number-address ${EVEN_NUMBER_ADDRESS:?} --number 4 --program-url https://plum-accurate-weasel-904.mypinata.cloud/ipfs/QmU7eqsYWguHCYGQzcg42faQQkgRfWScig7BcsdM1sJciw
```

### Uploading your own guest program

If you want to upload your own modified version of the zkVM guest, you'll need to set up [Pinata](https://pinata.cloud/) (which has a free tier). Create an account, generate an API key, and set the JWT as an environment variable:

```bash
export PINATA_JWT="YOUR_PINATA_JWT"
```

Then run without the `--program-url` flag:

```bash
RUST_LOG=info cargo run --bin app -- --rpc-url ${RPC_URL:?} --private-key ${PRIVATE_KEY:?} --even-number-address ${EVEN_NUMBER_ADDRESS:?} --number 4
```

## Local Development 

### Build

To build the example run:

```bash
# Install RISC Zero toolchain if not already installed
curl -L https://risczero.com/install | bash
rzup install

forge init --template https://github.com/boundless-xyz/boundless-foundry-template boundless-foundry-template
cargo build
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
