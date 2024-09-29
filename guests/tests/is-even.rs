// Copyright (c) 2024 RISC Zero, Inc.
//
// All rights reserved.

use alloy_primitives::U256;
use alloy_sol_types::SolValue;
use guests::IS_EVEN_ELF;
use risc0_zkvm::{default_executor, ExecutorEnv};

#[test]
fn proves_even_number() {
    let even_number = U256::from(1304);

    let env = ExecutorEnv::builder()
        .write_slice(&even_number.abi_encode())
        .build()
        .unwrap();

    // NOTE: Use the executor to run tests without proving.
    let session_info = default_executor().execute(env, IS_EVEN_ELF).unwrap();

    let x = U256::abi_decode(&session_info.journal.bytes, true).unwrap();
    assert_eq!(x, even_number);
}

#[test]
#[should_panic(expected = "number is not even")]
fn rejects_odd_number() {
    let odd_number = U256::from(75);

    let env = ExecutorEnv::builder()
        .write_slice(&odd_number.abi_encode())
        .build()
        .unwrap();

    // NOTE: Use the executor to run tests without proving.
    default_executor().execute(env, IS_EVEN_ELF).unwrap();
}
