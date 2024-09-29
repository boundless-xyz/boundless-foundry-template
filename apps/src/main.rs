// Copyright (c) 2024 RISC Zero, Inc.
//
// All rights reserved.

use std::time::Duration;

use crate::even_number::IEvenNumber::IEvenNumberInstance;
use alloy::{
    primitives::{aliases::U96, utils::parse_ether, Address, U256},
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
};
use anyhow::{Context, Result};
use boundless_market::{
    contracts::{Input, Offer, Predicate, ProvingRequest, Requirements},
    sdk::client::Client,
};
use clap::Parser;
use guests::{IS_EVEN_ELF, IS_EVEN_ID};
use risc0_zkvm::{default_executor, sha::Digestible, ExecutorEnv};
use url::Url;

/// Timeout for the EVM RPC transaction to be confirmed.
pub const TX_TIMEOUT: Duration = Duration::from_secs(3);

mod even_number {
    alloy::sol!(
        #![sol(rpc, all_derives)]
        "../contracts/src/IEvenNumber.sol"
    );
}

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// URL of the Ethereum RPC endpoint.
    #[clap(short, long, env)]
    rpc_url: Url,
    /// Private key used to interact with the EvenNumber contract.
    #[clap(short, long, env)]
    wallet_private_key: PrivateKeySigner,
    /// Address of the EvenNumber contract.
    #[clap(short, long, env)]
    even_number_address: Address,
    /// Address of the SetVerifier contract.
    #[clap(short, long, env)]
    set_verifier_address: Address,
    /// Address of the ProofMarket contract.
    #[clap(short, long, env)]
    proof_market_address: Address,
    /// The input to provide to the guest binary
    #[clap(short, long)]
    input: U256,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    dotenvy::dotenv()?;
    let args = Args::parse();

    // NOTE: Using a separate `run` function to facilitate testing below.
    run(
        args.wallet_private_key,
        args.rpc_url,
        args.proof_market_address,
        args.set_verifier_address,
        args.even_number_address,
        args.input,
    )
    .await?;

    Ok(())
}

async fn run(
    wallet_private_key: PrivateKeySigner,
    rpc_url: Url,
    proof_market_address: Address,
    set_verifier_address: Address,
    even_number_address: Address,
    input: U256,
) -> Result<()> {
    // Create a Boundless client from the provided parameters.
    let boundless_client = Client::from_parts(
        wallet_private_key,
        rpc_url,
        proof_market_address,
        set_verifier_address,
    )
    .await?;

    // Upload the ELF to the storage provider so that it can be fetched by the market.
    let image_url = boundless_client.upload_image(IS_EVEN_ELF).await?;
    tracing::info!("Uploaded image to {}", image_url);

    // Encode the input and upload it to the storage provider.
    let encoded_input = &input.abi_encode();
    let input_url = boundless_client.upload_input(&encoded_input).await?;
    tracing::info!("Uploaded input to {}", input_url);

    // Dry run the ELF with the input to get the journal and cycle count.
    // This can be useful to estimate the cost of the proving request.
    // It can also be useful to ensure the guest can be executed correctly and we do not send into
    // the market unprovable proving requests. If you have a different mechanism to get the expected
    // journal and set a price, you can skip this step.
    let env = ExecutorEnv::builder()
        .write_slice(&input.abi_encode())
        .build()?;
    let session_info = default_executor().execute(env, IS_EVEN_ELF)?;
    let mcycles_count = session_info
        .segments
        .iter()
        .map(|segment| 1 << segment.po2)
        .sum::<u64>()
        .div_ceil(1_000_000);
    let required_journal = session_info.journal;

    // Create a proving request with the image, input, requirements and offer.
    // The ELF (i.e. image) is specified by the image URL.
    // The input can be specified by an URL, as in this example, or can be posted on chain by using
    // the `with_inline` method with the input bytes.
    // The requirements are the IS_EVEN_ID and the digest of the journal. In this way, the market can
    // verify that the proof is correct by checking both the committed image id and digest of the
    // journal. The offer specifies the price range and the timeout for the request.
    // Additionally, the offer can also specify:
    // - the bidding start time: the block number when the bidding starts;
    // - the ramp up period: the number of blocks before the price start increasing until reaches
    //   the maxPrice, starting from the the bidding start;
    // - the lockin price: the price at which the request can be locked in by a prover, if the
    //   request is not fulfilled before the timeout, the prover can be slashed.
    let request = ProvingRequest::default()
        .with_image_url(&image_url)
        .with_input(Input::url(&input_url))
        .with_requirements(Requirements::new(
            IS_EVEN_ID,
            Predicate::digest_match(required_journal.digest()),
        ))
        .with_offer(
            Offer::default()
                // The market uses a reverse Dutch auction mechanism to match requests with provers.
                // Each request has a price range that a prover can bid on. One way to set the price
                // is to choose a desired (min and max) price per million cycles and multiply it
                // by the number of cycles. Alternatively, you can use the `with_min_price` and
                // `with_max_price` methods to set the price directly.
                .with_min_price_per_mcycle(
                    U96::from::<u128>(parse_ether("0.001")?.try_into()?),
                    mcycles_count,
                )
                // NOTE: If your offer is not being accepted, try increasing the max price.
                .with_max_price_per_mcycle(
                    U96::from::<u128>(parse_ether("0.002")?.try_into()?),
                    mcycles_count,
                )
                // The timeout is the maximum number of blocks the request can stay
                // unfulfilled in the market before it expires. If a prover locks in
                // the request and does not fulfill it before the timeout, the prover can be
                // slashed.
                .with_timeout(1000),
        );

    // Send the request and wait for it to be completed.
    let request_id = boundless_client.submit_request(&request).await?;
    tracing::info!("Request {} submitted", request_id);

    // Wait for the request to be fulfilled by the market.
    // We already calculated a [required_journal],
    // and use it over the journal provided, and extract only the seal.
    tracing::info!("Waiting for request {} to be fulfilled", request_id);
    let (_ignored_returned_journal, seal) = boundless_client
        .wait_for_request_fulfillment(request_id, Duration::from_secs(5), None)
        .await?;
    tracing::info!("Request {} fulfilled", request_id);

    // Interact with the EvenNumber contract by calling the set function with the journal and
    // seal returned by the market.
    let even_number_instance =
        IEvenNumberInstance::new(even_number_address, boundless_client.provider().clone());
    let encoded_number = U256::from_be_slice(&required_journal.bytes);
    let set_number = even_number_instance
        .set(encoded_number, seal)
        .from(boundless_client.caller());

    // By calling the set function, we verify the seal against the published roots
    // of the SetVerifier contract.
    tracing::info!("Calling EvenNumber set function");
    let pending_tx = set_number.send().await.context("failed to broadcast tx")?;
    tracing::info!("Broadcasting tx {}", pending_tx.tx_hash());
    let tx_hash = pending_tx
        .with_timeout(Some(TX_TIMEOUT))
        .watch()
        .await
        .context("failed to confirm tx")?;
    tracing::info!("Tx {:?} confirmed", tx_hash);

    // We query the value stored at the EvenNumber address to check it was set correctly
    let number = even_number_instance
        .get()
        .call()
        .await
        .with_context(|| format!("failed to get number"))?
        ._0;
    tracing::info!(
        "Number for address: {:?} is set to {:?}",
        boundless_client.caller(),
        number
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use alloy::{
        network::EthereumWallet,
        node_bindings::{Anvil, AnvilInstance},
        primitives::Address,
        providers::ProviderBuilder,
        signers::local::PrivateKeySigner,
    };
    use boundless_market::contracts::test_utils::TestCtx;
    use broker::broker_from_test_ctx;
    use tokio::time::timeout;
    use tracing_test::traced_test;

    use super::*;

    /// Test Timeout for the entire [run] to complete.
    /// Should be greater than [TX_TIMEOUT].
    /// Heuristic: ~30 seconds needed required to finish test.
    const RUN_TIMEOUT: Duration = Duration::from_secs(60);

    alloy::sol!(
        #![sol(rpc)]
        EvenNumber,
        "../contracts/out/EvenNumber.sol/EvenNumber.json"
    );

    async fn deploy_even_number(anvil: &AnvilInstance, test_ctx: &TestCtx) -> Result<Address> {
        let deployer_signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let deployer_provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(EthereumWallet::from(deployer_signer))
            .on_builtin(&anvil.endpoint())
            .await
            .unwrap();
        let even_number =
            EvenNumber::deploy(&deployer_provider, test_ctx.set_verifier_addr).await?;

        Ok(*even_number.address())
    }

    #[tokio::test]
    #[traced_test]
    // This test should run in dev mode, otherwise a storage provider and a prover backend are
    // required. To run in dev mode, set the `RISC0_DEV_MODE` environment variable to `true`,
    // e.g.: `RISC0_DEV_MODE=true cargo test`
    async fn test_main() {
        // Define input to guest
        let input = U256::from(42);

        // Setup anvil and deploy contracts
        let anvil = Anvil::new().spawn();
        let ctx = TestCtx::new(&anvil).await.unwrap();
        let even_number_address = deploy_even_number(&anvil, &ctx).await.unwrap();

        // Start a broker
        let broker = broker_from_test_ctx(&ctx, anvil.endpoint_url())
            .await
            .unwrap();
        let broker_task = tokio::spawn(async move {
            broker.start_service().await.unwrap();
        });

        // Run the main function with a timeout
        let result = timeout(
            RUN_TIMEOUT,
            run(
                ctx.customer_signer,
                anvil.endpoint_url(),
                ctx.proof_market_addr,
                ctx.set_verifier_addr,
                even_number_address,
                input,
            ),
        )
        .await;

        tracing::info!("{result:?}");

        // Check the result of the timeout
        match result {
            Ok(run_result) => {
                // If the run completed, check for errors
                run_result.unwrap();
            }
            Err(_) => {
                // If timeout occurred, abort the broker task and fail the test
                broker_task.abort();
                panic!(
                    "The run function did not complete within {:?} seconds.",
                    RUN_TIMEOUT.as_secs()
                );
            }
        }

        // Check for a broker panic
        if broker_task.is_finished() {
            broker_task.await.unwrap();
        } else {
            broker_task.abort();
        }
    }
}
