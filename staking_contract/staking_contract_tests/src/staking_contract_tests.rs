use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, runtime_args, ContractHash, ContractPackageHash, Key,
    PublicKey, RuntimeArgs, SecretKey, BLAKE2B_DIGEST_LENGTH, U256,
};
use once_cell::sync::Lazy;
use std::convert::TryInto;
use std::time::SystemTime;

const ADDRESS: &str = "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d";

const ERC20_WASM: &str = "erc20.wasm";
const STAKING_WASM: &str = "staking_contract.wasm";
const ERC20_CONTRACT_NAME: &str = "erc20_token_contract";
const ERC20_CONTRACT_PACKAGE_HASH: &str = "erc20-contract_package_hash";
const STAKING_CONTRACT_HASH: &str = "staking_contract_hash";
const STAKING_CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";
const ALLOWANCES_SEED_UREF: &str = "allowances";

#[test]
fn test_approve_and_stake() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_package_hash {:?}",
        erc20_contract_package_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" => 1681708875776u64,
        "withdraw_starts" => 1681708875776u64,
        "withdraw_ends" => 1781708875776u64,
        "staking_total" => U256::from(500000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let approve_args = runtime_args! {
        "spender" => staking_contract_key,
        "amount" => U256::from(10i64),
    };

    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "approve",
        approve_args,
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();
}

#[test]
#[should_panic]
fn test_stake_but_not_approve() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_package_hash {:?}",
        erc20_contract_package_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" => 1781708875776u64,
        "withdraw_starts" => 1781708875776u64,
        "withdraw_ends" => 1781708875776u64,
        "staking_total" => U256::from(500000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();
}

#[test]
fn test_approve_and_stake_and_amount_staked() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_hash {:?}",
        erc20_contract_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" => 1681708875776u64,
        "withdraw_starts" => 1681708875776u64,
        "withdraw_ends" => 1781708875776u64,
        "staking_total" => U256::from(500000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let approve_args = runtime_args! {
        "spender" => staking_contract_key,
        "amount" => U256::from(10i64),
    };

    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "approve",
        approve_args,
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();

    let amount_staked_args = runtime_args! {
        "staker" => Key::from(*DEFAULT_ACCOUNT_ADDR),
    };

    let amount_staked_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "amount_staked",
        amount_staked_args,
    )
    .build();

    builder
        .exec(amount_staked_request)
        .expect_success()
        .commit();
}

#[test]
#[should_panic]
fn test_approve_and_stake_and_amount_staked_wrong_address() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_hash {:?}",
        erc20_contract_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" => 1781708875776u64,
        "withdraw_starts" => 1781708875776u64,
        "withdraw_ends" => 1781708875776u64,
        "staking_total" => U256::from(500000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let approve_args = runtime_args! {
        "spender" => staking_contract_key,
        "amount" => U256::from(10i64),
    };

    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "approve",
        approve_args,
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();

    let amount_staked_args = runtime_args! {
        "staker" => Key::from(*Lazy::new(|| AccountHash::from(&*Lazy::new(|| {
            let secret_key = SecretKey::ed25519_from_bytes([200; SecretKey::ED25519_LENGTH]).unwrap();
            PublicKey::from(&secret_key)
        })))),
    };

    let amount_staked_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "amount_staked",
        amount_staked_args,
    )
    .build();

    builder
        .exec(amount_staked_request)
        .expect_success()
        .commit();
}

#[test]
fn test_approve_and_stake_and_staker_reward() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_hash {:?}",
        erc20_contract_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" => 1781708875776u64,
        "withdraw_starts" => 1781708875776u64,
        "withdraw_ends" => 1781708875786u64,
        "staking_total" => U256::from(500000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let approve_args = runtime_args! {
        "spender" => staking_contract_key,
        "amount" => U256::from(10i64),
    };

    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "approve",
        approve_args,
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();

    let balance_of_args = runtime_args! {
        "address" => staking_contract_key,
    };

    let balance_of_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "balance_of",
        balance_of_args,
    )
    .build();

    builder
        .exec(balance_of_request)
        .expect_success()
        .commit()
        .get_exec_results();

    let staker_reward_args = runtime_args! {
        "staker_address" => Key::from(*DEFAULT_ACCOUNT_ADDR),
    };

    let staker_reward_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "staker_reward",
        staker_reward_args,
    )
    .with_block_time(1781708875776u64)
    .build();

    builder
        .exec(staker_reward_request)
        .expect_success()
        .commit();
}

#[test]
#[should_panic]
fn test_approve_and_stake_and_staker_reward_wrong_address() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_hash {:?}",
        erc20_contract_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" => 1781708875776u64,
        "withdraw_starts" => 1781708875776u64,
        "withdraw_ends" => 1781708875776u64,
        "staking_total" => U256::from(500000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let approve_args = runtime_args! {
        "spender" => staking_contract_key,
        "amount" => U256::from(10i64),
    };

    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "approve",
        approve_args,
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();

    let balance_of_args = runtime_args! {
        "address" => staking_contract_key,
    };

    let balance_of_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "balance_of",
        balance_of_args,
    )
    .build();

    builder
        .exec(balance_of_request)
        .expect_success()
        .commit()
        .get_exec_results();

    let staker_reward_args = runtime_args! {
        "staker_address" => Key::from(*Lazy::new(|| AccountHash::from(&*Lazy::new(|| {
            let secret_key = SecretKey::ed25519_from_bytes([200; SecretKey::ED25519_LENGTH]).unwrap();
            PublicKey::from(&secret_key)
        })))),
    };

    let staker_reward_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "staker_reward",
        staker_reward_args,
    )
    .build();

    builder
        .exec(staker_reward_request)
        .expect_success()
        .commit();
}

#[test]
fn test_approve_and_stake_and_get_current_reward() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_hash {:?}",
        erc20_contract_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" => 1681708875776u64,
        "withdraw_starts" => 1681708875776u64,
        "withdraw_ends" => 1781708875776u64,
        "staking_total" => U256::from(500000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let approve_args = runtime_args! {
        "spender" => staking_contract_key,
        "amount" => U256::from(10i64),
    };

    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "approve",
        approve_args,
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();

    let balance_of_args = runtime_args! {
        "address" => staking_contract_key,
    };

    let balance_of_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "balance_of",
        balance_of_args,
    )
    .build();

    builder
        .exec(balance_of_request)
        .expect_success()
        .commit()
        .get_exec_results();

    let get_current_reward_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "get_current_reward",
        runtime_args! {},
    )
    .build();

    builder
        .exec(get_current_reward_request)
        .expect_success()
        .commit();
}

#[test]
fn test_approve_and_stake_and_withdraw() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_hash {:?}",
        erc20_contract_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" =>    1781708875779u64,
        "withdraw_starts" => 1781708875779u64,
        "withdraw_ends" =>   1781708875779u64,
        "staking_total" => U256::from(50000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let approve_args = runtime_args! {
        "spender" => staking_contract_key,
        "amount" => U256::from(10i64),
    };

    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "approve",
        approve_args,
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();

    let withdraw_args = runtime_args! {
        "amount" => U256::from(1u64),
    };

    let withdraw_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "withdraw",
        withdraw_args,
    )
    .with_block_time(1781708875779u64)
    .build();

    builder.exec(withdraw_request).expect_success().commit();
}

#[test]
#[should_panic]
fn test_approve_and_stake_and_withdraw_too_big_amount() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_hash {:?}",
        erc20_contract_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" => 1781708875776u64,
        "withdraw_starts" => 1781708875776u64,
        "withdraw_ends" => 1781708875776u64,
        "staking_total" => U256::from(500000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let approve_args = runtime_args! {
        "spender" => staking_contract_key,
        "amount" => U256::from(10i64),
    };

    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "approve",
        approve_args,
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();

    let withdraw_args = runtime_args! {
        "amount" => U256::from(10u64),
    };

    let withdraw_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "withdraw",
        withdraw_args,
    )
    .build();

    builder.exec(withdraw_request).expect_success().commit();
}

#[test]
fn test_approve_and_stake_and_add_reward() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_hash {:?}",
        erc20_contract_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" => 1681708875776u64,
        "withdraw_starts" => 1681708875776u64,
        "withdraw_ends" => 1781708875776u64,
        "staking_total" => U256::from(500000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let approve_args = runtime_args! {
        "spender" => staking_contract_key,
        "amount" => U256::from(10i64),
    };

    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "approve",
        approve_args,
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();

    let add_reward_args = runtime_args! {
        "reward_amount" => U256::from(1i64),
        "withdrawable_amount" => U256::from(1i64),
    };

    let add_reward_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "add_reward",
        add_reward_args,
    )
    .build();

    builder.exec(add_reward_request).expect_success().commit();
}

#[test]
#[should_panic]
fn test_approve_and_stake_and_add_reward_withdrawable_amount_too_big() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let erc20_runtime_args = runtime_args! {
        "name" => "FERRUM_ERC20".to_string(),
        "symbol" => "F_ERC20".to_string(),
        "total_supply" => U256::from(500000i64),
        "decimals" => 8u8,
    };

    let erc_20_install_request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
            .build();

    builder
        .exec(erc_20_install_request)
        .expect_success()
        .commit();

    let erc20_contract_hash = get_erc20_contract_hash(&builder);
    let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

    println!(
        "erc20_contract_hash {:?}",
        erc20_contract_hash.to_formatted_string()
    );
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "mint",
        runtime_args! {},
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    let erc20_contract_key: Key = erc20_contract_hash.into();

    let balance = balance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(balance, U256::from(510000u64));

    let staking_contract_runtime_args = runtime_args! {
        "name" => "FerrumX".to_string(),
        "address" => "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d".to_string(),
        "staking_starts" => 0u64,
        "staking_ends" => 1681708875776u64,
        "withdraw_starts" => 1681708875776u64,
        "withdraw_ends" => 1781708875776u64,
        "staking_total" => U256::from(500000i64),
        "erc20_contract_package_hash" => Key::from(erc20_contract_package_hash),
    };

    let staking_contract_install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        STAKING_WASM,
        staking_contract_runtime_args,
    )
    .build();

    builder
        .exec(staking_contract_install_request)
        .expect_success()
        .commit();

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_package_hash = get_stacking_contract_package_hash(&builder);

    let staking_contract_hash = get_stacking_contract_hash(&builder);

    let staking_contract_key: Key = staking_contract_package_hash.into();

    let approve_args = runtime_args! {
        "spender" => staking_contract_key,
        "amount" => U256::from(10i64),
    };

    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        erc20_contract_hash,
        "approve",
        approve_args,
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let actual_allowance = allowance_dictionary(
        &builder,
        erc20_contract_key,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        staking_contract_key,
    );

    assert_eq!(actual_allowance, U256::from(10i64));

    let stake_args = runtime_args! {
        "amount" => U256::from(5i64),
        "staking_contract_package_hash" => get_stacking_contract_package_hash(&builder).to_formatted_string(),
    };

    let stake_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "stake",
        stake_args,
    )
    .build();

    builder.exec(stake_request).expect_success().commit();

    let add_reward_args = runtime_args! {
        "reward_amount" => U256::from(1i64),
        "withdrawable_amount" => U256::from(2i64),
    };

    let add_reward_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        staking_contract_hash,
        "add_reward",
        add_reward_args,
    )
    .build();

    builder.exec(add_reward_request).expect_success().commit();
}

/// Creates a dictionary item key for an (owner, spender) pair.
fn make_allowances_dictionary_item_key(owner: Key, spender: Key) -> String {
    let mut preimage = Vec::new();
    preimage.append(&mut owner.to_bytes().unwrap());
    preimage.append(&mut spender.to_bytes().unwrap());

    let key_bytes = create_blake2b_hash(&preimage);
    hex::encode(&key_bytes)
}

pub(crate) fn create_blake2b_hash<T: AsRef<[u8]>>(data: T) -> [u8; BLAKE2B_DIGEST_LENGTH] {
    let mut result = [0; BLAKE2B_DIGEST_LENGTH];
    // NOTE: Assumed safe as `BLAKE2B_DIGEST_LENGTH` is a valid value for a hasher
    let mut hasher = VarBlake2b::new(BLAKE2B_DIGEST_LENGTH).expect("should create hasher");

    hasher.update(data);
    hasher.finalize_variable(|slice| {
        result.copy_from_slice(slice);
    });
    result
}

pub fn get_stacking_contract_package_hash(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
) -> ContractPackageHash {
    let erc20_hash_addr = builder
        .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
        .named_keys()
        .get(STAKING_CONTRACT_PACKAGE_HASH)
        .expect("must have this entry in named keys")
        .into_hash()
        .expect("must get hash_addr");

    ContractPackageHash::new(erc20_hash_addr)
}

pub fn get_stacking_contract_hash(builder: &WasmTestBuilder<InMemoryGlobalState>) -> ContractHash {
    let erc20_hash_addr = builder
        .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
        .named_keys()
        .get(STAKING_CONTRACT_HASH)
        .expect("must have this entry in named keys")
        .into_hash()
        .expect("must get hash_addr");

    ContractHash::new(erc20_hash_addr)
}

pub(crate) fn get_erc20_contract_hash(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
) -> ContractHash {
    let erc20_hash_addr = builder
        .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
        .named_keys()
        .get(ERC20_CONTRACT_NAME)
        .expect("must have this entry in named keys")
        .into_hash()
        .expect("must get hash_addr");

    ContractHash::new(erc20_hash_addr)
}

pub(crate) fn get_erc20_contract_package_hash(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
) -> ContractPackageHash {
    let erc20_hash_addr = builder
        .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
        .named_keys()
        .get(ERC20_CONTRACT_PACKAGE_HASH)
        .expect("must have this entry in named keys")
        .into_hash()
        .expect("must get hash_addr");

    ContractPackageHash::new(erc20_hash_addr)
}

fn balance_dictionary(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
    erc20_contract_key: Key,
    owner_key: Key,
) -> U256 {
    let balance_seed_uref = builder
        .query(None, erc20_contract_key, &vec![])
        .unwrap()
        .as_contract()
        .expect("must have ERC20 contract")
        .named_keys()
        .get("balances")
        .expect("must have balances entry")
        .as_uref()
        .expect("must be a uref")
        .to_owned();

    let dict_item_key = make_dictionary_item_key(owner_key);

    let balance = builder
        .query_dictionary_item(None, balance_seed_uref, &dict_item_key)
        .expect("should be stored value.")
        .as_cl_value()
        .expect("should be cl value.")
        .clone()
        .into_t()
        .expect("must convert to U256");

    balance
}

fn allowance_dictionary(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
    erc20_contract_key: Key,
    owner_key: Key,
    spender_key: Key,
) -> U256 {
    let allowance_seed_uref = builder
        .query(None, erc20_contract_key, &vec![])
        .unwrap()
        .as_contract()
        .expect("must have ERC20 contract")
        .named_keys()
        .get(ALLOWANCES_SEED_UREF)
        .expect("must have allowances entry")
        .as_uref()
        .expect("must be a uref")
        .to_owned();

    let dict_item_key = make_allowances_dictionary_item_key(owner_key, spender_key);

    let allowance = builder
        .query_dictionary_item(None, allowance_seed_uref, &dict_item_key)
        .expect("should be stored value.")
        .as_cl_value()
        .expect("should be cl value.")
        .clone()
        .into_t()
        .expect("must convert to U256");

    allowance
}

fn make_dictionary_item_key(owner: Key) -> String {
    let preimage = owner.to_bytes().unwrap();
    // NOTE: As for now dictionary item keys are limited to 64 characters only. Instead of using
    // hashing (which will effectively hash a hash) we'll use base64. Preimage is about 33 bytes for
    // both Address variants, and approximated base64-encoded length will be 4 * (33 / 3) ~ 44
    // characters.
    // Even if the preimage increased in size we still have extra space but even in case of much
    // larger preimage we can switch to base85 which has ratio of 4:5.
    base64::encode(&preimage)
}
