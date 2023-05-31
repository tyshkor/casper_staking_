#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// External dependencies
extern crate alloc;

use alloc::vec;
use alloc::{collections::BTreeSet, format, string::String};

// Contract API dependencies
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, CLValue, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};

// Custom dependencies
use contract_utils::{ContractContext, OnChainContractStorage};
use staking_contract::staking_contract::CEP20STK;

/// Struct representing the token contract.
#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl CEP20STK<OnChainContractStorage> for Token {}

impl Token {
    /// Constructor for the token contract.
    #[allow(clippy::too_many_arguments)]
    fn constructor(
        &mut self,
        name: String,
        address: String,
        staking_starts: u64,
        staking_ends: u64,
        withdraw_starts: u64,
        withdraw_ends: u64,
        staking_total: U256,
    ) {
        CEP20STK::init(
            self,
            name,
            address,
            staking_starts,
            staking_ends,
            withdraw_starts,
            withdraw_ends,
            staking_total,
        );
    }
}

// The constructor function takes the following arguments:
// - name: The name of the staking contract.
// - address: The address of the staking contract.
// - staking_starts: The start date of the staking period.
// - staking_ends: The end date of the staking period.
// - withdraw_starts: The start date of the withdrawal period.
// - withdraw_ends: The end date of the withdrawal period.
// - staking_total: The total number of tokens that can be staked.
#[no_mangle]
pub extern "C" fn constructor() {
    // Read arguments for the constructor call.
    let name = runtime::get_named_arg::<String>("name");
    let address = runtime::get_named_arg::<String>("address");
    let staking_starts: u64 = runtime::get_named_arg::<u64>("staking_starts");
    let staking_ends: u64 = runtime::get_named_arg::<u64>("staking_ends");
    let withdraw_starts: u64 = runtime::get_named_arg::<u64>("withdraw_starts");
    let withdraw_ends: u64 = runtime::get_named_arg::<u64>("withdraw_ends");
    let staking_total: U256 = runtime::get_named_arg::<U256>("staking_total");
    let stacking_contract_package_hash =
        runtime::get_named_arg::<Key>("stacking_contract_package_hash");
    let erc20_contract_package_hash = runtime::get_named_arg::<Key>("erc20_contract_package_hash");

    // Store the stacking_contract_package_hash and erc20_contract_package_hash as keys
    #[allow(clippy::useless_conversion)]
    runtime::put_key(
        "stacking_contract_package_hash",
        stacking_contract_package_hash.into(),
    );

    #[allow(clippy::useless_conversion)]
    runtime::put_key(
        "erc20_contract_package_hash",
        erc20_contract_package_hash.into(),
    );

    // Initialize the token contract using the constructor arguments
    Token::default().constructor(
        name,
        address,
        staking_starts,
        staking_ends,
        withdraw_starts,
        withdraw_ends,
        staking_total,
    );
}

// The `name` function returns the name of the staking contract.
#[no_mangle]
pub extern "C" fn name() {
    let ret = Token::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `address` function returns the address of the staking contract.
#[no_mangle]
pub extern "C" fn address() {
    let ret = Token::default().address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `staking_starts` function returns the start date of the staking period.
#[no_mangle]
pub extern "C" fn staking_starts() {
    let ret = Token::default().staking_starts();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `staking_ends` function returns the end date of the staking period.
#[no_mangle]
pub extern "C" fn staking_ends() {
    let ret = Token::default().staking_starts();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `withdraw_starts` function returns the start date of the withdrawal period.
#[no_mangle]
pub extern "C" fn withdraw_starts() {
    let ret = Token::default().withdraw_starts();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `withdraw_ends` function returns the end date of the withdrawal period.
#[no_mangle]
pub extern "C" fn withdraw_ends() {
    let ret = Token::default().withdraw_ends();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `staking_total` function returns the total number of tokens that can be staked.
#[no_mangle]
pub extern "C" fn staking_total() {
    let ret = Token::default().staking_total();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `amount_staked` function takes the following argument:
// - staker: The address of the staker.
// The function returns the number of tokens that the staker has staked.
#[no_mangle]
pub extern "C" fn amount_staked() {
    let staker = runtime::get_named_arg::<Key>("staker");
    let ret = Token::default().amount_staked(staker).unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `stake` function takes the following arguments:
// - amount: The number of tokens to stake.
// - staking_contract_package_hash: The hash of the staking contract package.
// The function stakes the specified number of tokens in the staking contract.
#[no_mangle]
pub extern "C" fn stake() {
    let amount = runtime::get_named_arg::<U256>("amount");
    let staking_contract_package_hash =
        runtime::get_named_arg::<String>("staking_contract_package_hash");
    let ret = Token::default()
        .stake(amount, staking_contract_package_hash)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `withdraw` function takes the following arguments:
// - amount: The number of tokens to withdraw.
// The function withdraws the specified number of tokens from the staking contract.
#[no_mangle]
pub extern "C" fn withdraw() {
    let amount = runtime::get_named_arg::<U256>("amount");
    let ret = Token::default().withdraw(amount).unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `add_reward` function takes the following arguments:
// - reward_amount: The amount of reward to add.
// - withdrawable_amount: The amount of reward that is now withdrawable.
// The function adds the specified amount of reward to the staking contract and updates the withdrawable amount.
#[no_mangle]
pub extern "C" fn add_reward() {
    let reward_amount = runtime::get_named_arg::<U256>("reward_amount");
    let withdrawable_amount = runtime::get_named_arg::<U256>("withdrawable_amount");
    let ret = Token::default()
        .add_reward(reward_amount, withdrawable_amount)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `get_current_reward` function returns the current reward amount.
#[no_mangle]
pub extern "C" fn get_current_reward() {
    let ret = Token::default().reward_balance();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `staker_reward` function takes the following argument:
// - staker_address: The address of the staker.
// The function returns the reward amount that the staker has earned.
#[no_mangle]
pub extern "C" fn staker_reward() {
    let staker_address = runtime::get_named_arg::<Key>("staker_address");
    let ret = Token::default()
        .staker_reward(staker_address)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    // Read arguments for the constructor call.
    let name: String = runtime::get_named_arg("name");
    let address = runtime::get_named_arg::<String>("address");
    let staking_starts: u64 = runtime::get_named_arg::<u64>("staking_starts");
    let staking_ends: u64 = runtime::get_named_arg::<u64>("staking_ends");
    let withdraw_starts: u64 = runtime::get_named_arg::<u64>("withdraw_starts");
    let withdraw_ends: u64 = runtime::get_named_arg::<u64>("withdraw_ends");
    let staking_total: U256 = runtime::get_named_arg::<U256>("staking_total");
    let erc20_contract_package_hash = runtime::get_named_arg::<Key>("erc20_contract_package_hash");

    /*

    casper-client put-deploy \
      --chain-name casper-test \
      --node-address http://44.208.234.65:7777 \
      --secret-key ./staking_contract/keys/secret_key.pem \
      --session-path ./staking_contract/target/wasm32-unknown-unknown/release/staking_contract.wasm \
      --payment-amount 200000000000 \
      --session-arg "name:string='FerrumX'" \
      --session-arg "address:string='782fe4b0bb944e6b1fd2c5a1456a78f0e2193d47dee9b1af5711d6b6e6aaca60'" \
      --session-arg "staking_starts:u64='0'" \
      --session-arg "staking_ends:u64='1755994649'" \
      --session-arg "withdraw_starts:u64='0'" \
      --session-arg "withdraw_ends:u64='1755994649'" \
      --session-arg "staking_total:U256='500000'" \
      --session-arg "erc20_contract_hash:string='contract-2934370f0a53457987d4dba9d68d71e7ee45b676677fbc66653bd15ea58db60f'"

        */
    // casper-client put-deploy \
    // --chain-name casper-test \
    // --node-address http://44.208.234.65:7777 \
    // --secret-key ./staking_contract/keys/secret_key.pem \
    // --session-path ./target/wasm32-unknown-unknown/release/staking_contract.wasm \
    // --payment-amount 200000000000 \
    // --session-arg "name:string='FerrumX'" \
    // --session-arg "address:string='782fe4b0bb944e6b1fd2c5a1456a78f0e2193d47dee9b1af5711d6b6e6aaca60'" \
    // --session-arg "staking_starts:u64='0'" \
    // --session-arg "staking_ends:u64='1755994649'" \
    // --session-arg "withdraw_starts:u64='0'" \
    // --session-arg "withdraw_ends:u64='1755994649'" \
    // --session-arg "staking_total:U256='500000'" \
    // --session-arg "erc20_contract_hash:string='contract-4277c47789fd154c42471978b4bffec31760f56b07507334a219682cf475181f'"

    /*
          casper-client put-deploy \
          --chain-name casper-test \
          --node-address http://44.208.234.65:7777 \
          --secret-key staking_contract/keys/secret_key.pem \
          --payment-amount 8000000000 \
          --session-hash 5ec37b076f618f8e3205eda7ad1f8bade197b296716b7e0cbba11a7b7c613bc6 \
          --session-entry-point "stake" \
          --session-arg "amount:u256='5'"
    */

    // casper-client put-deploy \
    // --chain-name casper-test \
    // --node-address http://44.208.234.65:7777 \
    // --secret-key staking_contract/keys/secret_key.pem \
    // --session-hash hash-4277c47789fd154c42471978b4bffec31760f56b07507334a219682cf475181f \
    // --session-entry-point approve \
    // --payment-amount 20000000000 \
    // --session-arg "spender:key='hash-153fc70a20eeefb8b6a02b217b6a19cc8439092569750ced554bde9d3d8f9896'" \
    // --session-arg "amount:u256='1000000000'"

    /*
          casper-client put-deploy \
          --chain-name casper-test \
          --node-address http://159.65.118.250:7777 \
          --secret-key cep47/keys/secret_key.pem \
          --payment-amount 4000000000 \
          --session-hash 6adb2902bf7c56116ead7ea7a2ffa269b8d4b117b632d2c44052f3c951dcaa0b \
          --session-entry-point "withdraw" \
          --session-arg "amount:u256='10'"
    */

    /*
          casper-client put-deploy \
          --chain-name casper-test \
          --node-address http://159.65.118.250:7777 \
          --secret-key cep47/keys/secret_key.pem \
          --payment-amount 4000000000 \
          --session-hash 6adb2902bf7c56116ead7ea7a2ffa269b8d4b117b632d2c44052f3c951dcaa0b \
          --session-entry-point "add_reward" \
          --session-arg "reward_amount:u256='20'" \
          --session-arg "withdrawable_amount:u256='19'"
    */

    /*
    casper-client get-deploy \
        --id 2 \
        --node-address http://159.65.118.250:7777 \
        ee07324ad466aad373e94f787b3dbf1ba1ff00175e97a0bce002bb45737ad5e6

    */

    let (contract_hash, _) = storage::new_contract(
        get_entry_points(),
        None,
        Some(String::from("contract_package_hash")),
        None,
    );

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key("contract_package_hash")
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    let package_hash_key: Key = package_hash.into();

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "name" => name,
        "address" => address,
        "staking_starts" => staking_starts,
        "staking_ends" => staking_ends,
        "withdraw_starts" => withdraw_starts,
        "withdraw_ends" => withdraw_ends,
        "staking_total" => staking_total,
        "stacking_contract_package_hash" => package_hash_key,
        "erc20_contract_package_hash" => erc20_contract_package_hash,
    };

    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    let _: () = runtime::call_contract(contract_hash, "constructor", constructor_args);

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    runtime::put_key("staking_contract_hash", contract_hash.into());
    runtime::put_key(
        &format!("{contract_hash}_contract_hash_wrapped"),
        storage::new_uref(contract_hash).into(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("address", String::cl_type()),
            Parameter::new("staking_starts", u64::cl_type()),
            Parameter::new("staking_ends", u64::cl_type()),
            Parameter::new("withdraw_starts", u64::cl_type()),
            Parameter::new("withdraw_ends", u64::cl_type()),
            Parameter::new("staking_total", U256::cl_type()),
            Parameter::new("erc20_contract_package_hash", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "address",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "staking_starts",
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw_starts",
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw_ends",
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "staking_total",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![Parameter::new("amount", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "stake",
        vec![
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("staking_contract_package_hash", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "amount_staked",
        vec![Parameter::new("staker", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_reward",
        vec![
            Parameter::new("reward_amount", Key::cl_type()),
            Parameter::new("withdrawable_amount", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_current_reward",
        vec![],
        <U256>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "staker_reward",
        vec![Parameter::new("staker_address", Key::cl_type())],
        <U256>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
