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

const ENTRY_POINT_NAME: &str = "name";
const ENTRY_POINT_ADDRESS: &str = "address";
const ENTRY_POINT_STAKING_STARTS: &str = "staking_starts";
const ENTRY_POINT_WITHDRAW_STARTS: &str = "withdraw_starts";
const ENTRY_POINT_WITHDRAW_ENDS: &str = "withdraw_ends";
const ENTRY_POINT_STAKING_TOTAL: &str = "staking_total";
const ENTRY_POINT_WITHDRAW: &str = "withdraw";
const ENTRY_POINT_STAKE: &str = "stake";
const ENTRY_POINT_ADD_REWARD: &str = "add_reward";
const ENTRY_POINT_AMOUNT_STAKED: &str = "amount_staked";
const ENTRY_POINT_GET_CURRENT_REWARD: &str = "get_current_reward";
const ENTRY_POINT_CONSTRUCTOR: &str = "constructor";
const ENTRY_POINT_STAKER_REWARD: &str = "staker_reward";

const AMOUNT: &str = "amount";
const STAKER: &str = "staker";
const ADDRESS: &str = "address";
const NAME: &str = "name";
const STAKING_STARTS: &str = "staking_starts";
const STAKING_ENDS: &str = "staking_ends";
const WITHDRAW_STARTS: &str = "withdraw_starts";
const WITHDRAW_ENDS: &str = "withdraw_ends";
const STAKING_TOTAL: &str = "staking_total";
const STACKING_CONTRACT_PACKAGE_HASH: &str = "stacking_contract_package_hash";
const ERC20_CONTRACT_PACKAGE_HASH: &str = "erc20_contract_package_hash";
const STAKER_ADDRESS: &str = "staker_address";
const WITHDRAWABLE_AMOUNT: &str = "withdrawable_amount";
const REWARD_AMOUNT: &str = "reward_amount";
const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";
const STAKING_CONTRACT_HASH: &str = "staking_contract_hash";

const CONSTRUCTOR_GROUP: &str = "constructor";

/// Struct representing the token contract.
#[derive(Default)]
struct Staking(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Staking {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl CEP20STK<OnChainContractStorage> for Staking {}

impl Staking {
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
        )
        .unwrap_or_revert();
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
    let name = runtime::get_named_arg::<String>(NAME);
    let address = runtime::get_named_arg::<String>(ADDRESS);
    let staking_starts: u64 = runtime::get_named_arg::<u64>(STAKING_STARTS);
    let staking_ends: u64 = runtime::get_named_arg::<u64>(STAKING_ENDS);
    let withdraw_starts: u64 = runtime::get_named_arg::<u64>(WITHDRAW_STARTS);
    let withdraw_ends: u64 = runtime::get_named_arg::<u64>(WITHDRAW_ENDS);
    let staking_total: U256 = runtime::get_named_arg::<U256>(STAKING_TOTAL);
    let stacking_contract_package_hash =
        runtime::get_named_arg::<Key>(STACKING_CONTRACT_PACKAGE_HASH);
    let erc20_contract_package_hash = runtime::get_named_arg::<Key>(ERC20_CONTRACT_PACKAGE_HASH);

    // Store the stacking_contract_package_hash and erc20_contract_package_hash as keys
    #[allow(clippy::useless_conversion)]
    runtime::put_key(
        STACKING_CONTRACT_PACKAGE_HASH,
        stacking_contract_package_hash.into(),
    );

    #[allow(clippy::useless_conversion)]
    runtime::put_key(
        ERC20_CONTRACT_PACKAGE_HASH,
        erc20_contract_package_hash.into(),
    );

    // Initialize the token contract using the constructor arguments
    Staking::default().constructor(
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
    let ret = Staking::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `address` function returns the address of the staking contract.
#[no_mangle]
pub extern "C" fn address() {
    let ret = Staking::default().address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `staking_starts` function returns the start date of the staking period.
#[no_mangle]
pub extern "C" fn staking_starts() {
    let ret = Staking::default().staking_starts();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `staking_ends` function returns the end date of the staking period.
#[no_mangle]
pub extern "C" fn staking_ends() {
    let ret = Staking::default().staking_starts();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `withdraw_starts` function returns the start date of the withdrawal period.
#[no_mangle]
pub extern "C" fn withdraw_starts() {
    let ret = Staking::default().withdraw_starts();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `withdraw_ends` function returns the end date of the withdrawal period.
#[no_mangle]
pub extern "C" fn withdraw_ends() {
    let ret = Staking::default().withdraw_ends();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `staking_total` function returns the total number of tokens that can be staked.
#[no_mangle]
pub extern "C" fn staking_total() {
    let ret = Staking::default().staking_total();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `amount_staked` function takes the following argument:
// - staker: The address of the staker.
// The function returns the number of tokens that the staker has staked.
#[no_mangle]
pub extern "C" fn amount_staked() {
    let staker = runtime::get_named_arg::<Key>(STAKER);
    let ret = Staking::default().amount_staked(staker).unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `stake` function takes the following arguments:
// - amount: The number of tokens to stake.
// The function stakes the specified number of tokens in the staking contract.
#[no_mangle]
pub extern "C" fn stake() {
    let amount = runtime::get_named_arg::<U256>(AMOUNT);
    let ret = Staking::default().stake(amount).unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `withdraw` function takes the following arguments:
// - amount: The number of tokens to withdraw.
// The function withdraws the specified number of tokens from the staking contract.
#[no_mangle]
pub extern "C" fn withdraw() {
    let amount = runtime::get_named_arg::<U256>(AMOUNT);
    let ret = Staking::default().withdraw(amount).unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `add_reward` function takes the following arguments:
// - reward_amount: The amount of reward to add.
// - withdrawable_amount: The amount of reward that is now withdrawable.
// The function adds the specified amount of reward to the staking contract and updates the withdrawable amount.
#[no_mangle]
pub extern "C" fn add_reward() {
    let reward_amount = runtime::get_named_arg::<U256>(REWARD_AMOUNT);
    let withdrawable_amount = runtime::get_named_arg::<U256>(WITHDRAWABLE_AMOUNT);
    let ret = Staking::default()
        .add_reward(reward_amount, withdrawable_amount)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `get_current_reward` function returns the current reward amount.
#[no_mangle]
pub extern "C" fn get_current_reward() {
    let ret = Staking::default().reward_balance();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// The `staker_reward` function takes the following argument:
// - staker_address: The address of the staker.
// The function returns the reward amount that the staker has earned.
#[no_mangle]
pub extern "C" fn staker_reward() {
    let staker_address = runtime::get_named_arg::<Key>(STAKER_ADDRESS);
    let ret = Staking::default()
        .staker_reward(staker_address)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    // Read arguments for the constructor call.
    let name: String = runtime::get_named_arg(NAME);
    let address = runtime::get_named_arg::<String>(ADDRESS);
    let staking_starts: u64 = runtime::get_named_arg::<u64>(STAKING_STARTS);
    let staking_ends: u64 = runtime::get_named_arg::<u64>(STAKING_ENDS);
    let withdraw_starts: u64 = runtime::get_named_arg::<u64>(WITHDRAW_STARTS);
    let withdraw_ends: u64 = runtime::get_named_arg::<u64>(WITHDRAW_ENDS);
    let staking_total: U256 = runtime::get_named_arg::<U256>(STAKING_TOTAL);
    let erc20_contract_package_hash = runtime::get_named_arg::<Key>(ERC20_CONTRACT_PACKAGE_HASH);

    let (contract_hash, _) = storage::new_contract(
        get_entry_points(),
        None,
        Some(String::from(CONTRACT_PACKAGE_HASH)),
        None,
    );

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key(CONTRACT_PACKAGE_HASH)
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    let package_hash_key: Key = package_hash.into();

    // Prepare constructor args
    let constructor_args = runtime_args! {
        NAME => name,
        ADDRESS => address,
        STAKING_STARTS => staking_starts,
        STAKING_ENDS => staking_ends,
        WITHDRAW_STARTS => withdraw_starts,
        WITHDRAW_ENDS => withdraw_ends,
        STAKING_TOTAL => staking_total,
        STACKING_CONTRACT_PACKAGE_HASH => package_hash_key,
        ERC20_CONTRACT_PACKAGE_HASH => erc20_contract_package_hash,
    };

    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, CONSTRUCTOR_GROUP, 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    let _: () = runtime::call_contract(contract_hash, ENTRY_POINT_CONSTRUCTOR, constructor_args);

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, CONSTRUCTOR_GROUP, urefs)
        .unwrap_or_revert();

    runtime::put_key(STAKING_CONTRACT_HASH, contract_hash.into());
    runtime::put_key(
        &format!("{contract_hash}_contract_hash_wrapped"),
        storage::new_uref(contract_hash).into(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_CONSTRUCTOR,
        vec![
            Parameter::new(NAME, String::cl_type()),
            Parameter::new(ADDRESS, String::cl_type()),
            Parameter::new(STAKING_STARTS, u64::cl_type()),
            Parameter::new(STAKING_ENDS, u64::cl_type()),
            Parameter::new(WITHDRAW_STARTS, u64::cl_type()),
            Parameter::new(WITHDRAW_ENDS, u64::cl_type()),
            Parameter::new(STAKING_TOTAL, U256::cl_type()),
            Parameter::new(ERC20_CONTRACT_PACKAGE_HASH, String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new(CONSTRUCTOR_GROUP)]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_NAME,
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_ADDRESS,
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_STAKING_STARTS,
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_WITHDRAW_STARTS,
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_WITHDRAW_ENDS,
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_STAKING_TOTAL,
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_WITHDRAW,
        vec![Parameter::new(AMOUNT, Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_STAKE,
        vec![Parameter::new(AMOUNT, U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_AMOUNT_STAKED,
        vec![Parameter::new(STAKER, Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_ADD_REWARD,
        vec![
            Parameter::new(REWARD_AMOUNT, Key::cl_type()),
            Parameter::new(WITHDRAWABLE_AMOUNT, Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_GET_CURRENT_REWARD,
        vec![],
        <U256>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_STAKER_REWARD,
        vec![Parameter::new(STAKER_ADDRESS, Key::cl_type())],
        <U256>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
