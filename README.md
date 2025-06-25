# Boundless Foundry Template

This template serves as a starter app powered by verifiable compute via [Boundless](https://docs.beboundless.xyz). 

It is built around a simple smart contract, `EvenNumber` deployed on Sepolia, and its associated RISC Zero guest, `is-even`. To get you started, we have deployed to [EvenNumber contract](https://sepolia.etherscan.io/address/0xE819474E78ad6e1C720a21250b9986e1f6A866A3#code) to Sepolia; we have also pre-uploaded the `is-even` guest to IPFS.

## Quick-start

1. [Install RISC Zero](https://dev.risczero.com/api/zkvm/install)

   ```sh
   curl -L https://risczero.com/install | bash
   rzup install
   ```

2. Clone this repo

   You can clone this repo with `git`, or use `forge init`:

   ```bash
   forge init --template https://github.com/boundless-xyz/boundless-foundry-template boundless-foundry-template
   ```
3. Set up your environment variables

   Export your Sepolia wallet private key as an environment variable (making sure it has enough funds):

   ```bash
   export RPC_URL="https://ethereum-sepolia-rpc.publicnode.com"
   export PRIVATE_KEY="YOUR_PRIVATE_KEY"
   ```

   You'll also need a deployment of the [EvenNumber contract](./contracts/src/EvenNumber.sol).
   You can use a predeployed contract on Sepolia:

   ```bash
   export EVEN_NUMBER_ADDRESS="0xE819474E78ad6e1C720a21250b9986e1f6A866A3"
   ```

4. Run the example app

   The [example app](apps/src/main.rs) will submit a request to the market for a proof that "4" is an even number, wait for the request to be fulfilled, and then submit that proof to the EvenNumber contract, setting the value to "4".

   To run the example using the pre-uploaded zkVM guest:

   ```bash
   RUST_LOG=info cargo run --bin app -- --number 4 --program-url https://plum-accurate-weasel-904.mypinata.cloud/ipfs/QmU7eqsYWguHCYGQzcg42faQQkgRfWScig7BcsdM1sJciw
   ```
## Development

### Build

To build the example run:

```
forge build
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

### Deploying the EvenNumber contract

You can deploy your smart contracts using forge script. To deploy the `EvenNumber` contract, run:

```
VERIFIER_ADDRESS="0x925d8331ddc0a1F0d96E68CF073DFE1d92b69187" forge script contracts/scripts/Deploy.s.sol --rpc-url ${RPC_URL:?} --broadcast -vv
export EVEN_NUMBER_ADDRESS=# address from the logs the script.
```

This will use the locally build guest binary, which you will need to upload using the steps below.

### Uploading your own guest program

When you modify your program, you'll need to upload your program to a public URL.
You can use any file hosting service, and the Boundless SDK provides built-in support uploading to AWS S3, and to IPFS via [Pinata](https://www.pinata.cloud/).

If you'd like to upload your program automatically using Pinata:

```bash
# The JWT from your Pinata account: https://app.pinata.cloud/developers/api-keys
export PINATA_JWT="YOUR_PINATA_JWT"
```

Then run without the `--program-url` flag:

```bash
RUST_LOG=info cargo run --bin app -- --number 4
```

You can also upload your program to any public URL ahead of time, and supply the URL via the `--program-url` flag.
