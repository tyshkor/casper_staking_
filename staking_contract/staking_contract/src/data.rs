use crate::error::Error;
use crate::event::StakingContractEvent;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime::get_call_stack, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, URef, U256};
use contract_utils::{get_key, key_to_str, set_key, Dict};
use core::convert::TryInto;

// Dictionary key for storing the amount staked by addresses
const AMOUNT_STAKED_BY_ADDRESS_DICT: &str = "amount_staked_by_addresses_dict";
// Dictionary key for storing the contract package hash
const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

// Keys used for accessing contract state
pub const NAME: &str = "name";
pub const ADDRESS: &str = "address";
pub const STAKING_STARTS: &str = "staking_starts";
pub const STAKING_ENDS: &str = "staking_ends";
pub const WITHDRAW_STARTS: &str = "withdraw_starts";
pub const WITHDRAW_ENDS: &str = "withdraw_ends";
pub const STAKING_TOTAL: &str = "staking_total";
pub const TOTAL_REWARD: &str = "total_reward";
pub const EARLY_WITHDRAW_REWARD: &str = "early_withdraw_reward";
pub const STAKED_TOTAL: &str = "staked_total";
pub const REWARD_BALANCE: &str = "reward_balance";
pub const STAKED_BALANCE: &str = "staked_balance";
const AMOUNT: &str = "amount";
const REWARD: &str = "reward";
const TOKEN_ADDRESS: &str = "token_address";
const STAKER_ADDRESS: &str = "staker_address";
const REWARD_AMOUNT: &str = "reward_amount";
const STAKED_AMOUNT: &str = "staked_amount";
const REQUESTED_AMOUNT: &str = "requested_amount";
const WITHDRAWABLE_AMOUNT: &str = "withdrawable_amount";

const EVENT_TYPE: &str = "event_type";

const STAKE: &str = "stake";
const PAID_OUT: &str = "paid_out";
const ADD_REWARD: &str = "add_reward";
const REFUNDED: &str = "refunded";

// Structure for managing staked tokens
pub struct StakedTokens {
    addresses_staked_dict: Dict,
}

impl StakedTokens {
    /// Creates a new instance of `StakedTokens`
    pub fn instance() -> StakedTokens {
        StakedTokens {
            addresses_staked_dict: Dict::instance(AMOUNT_STAKED_BY_ADDRESS_DICT),
        }
    }

    /// Initializes the `StakedTokens` dictionary
    pub fn init() {
        Dict::init(AMOUNT_STAKED_BY_ADDRESS_DICT);
    }

    /// Retrieves the amount staked by the given address
    pub fn get_amount_staked_by_address(&self, address: &Key) -> Option<U256> {
        self.addresses_staked_dict.get(&key_to_str(address))
    }

    /// Adds a stake for the owner
    pub fn add_stake(&self, owner: &Key, amount: &U256) {
        let new_amount = if let Some(staked_amount) = self.get_amount_staked_by_address(owner) {
            staked_amount + amount
        } else {
            *amount
        };
        self.addresses_staked_dict
            .set(&key_to_str(owner), new_amount);
    }

    /// Withdraws a stake for the owner
    pub fn withdraw_stake(&self, owner: &Key, amount: &U256) -> Result<(), Error> {
        let staked_amount = self
            .get_amount_staked_by_address(owner)
            .ok_or(Error::NotAStaker)?;
        let new_amount = staked_amount
            .checked_sub(*amount)
            .ok_or(Error::CheckedSub)?;
        self.addresses_staked_dict
            .set(&key_to_str(owner), new_amount);
        Ok(())
    }
}

/// Retrieves the stored name
pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

/// Sets the name
pub fn set_name(name: String) {
    set_key(NAME, name);
}

/// Retrieves the stored address
pub fn address() -> String {
    get_key(ADDRESS).unwrap_or_revert()
}

/// Sets the address
pub fn set_address(address: String) {
    set_key(ADDRESS, address);
}

/// Retrieves the staking start time
pub fn staking_starts() -> u64 {
    get_key(STAKING_STARTS).unwrap_or_revert()
}

/// Sets the staking start time
pub fn set_staking_starts(staking_starts: u64) {
    set_key(STAKING_STARTS, staking_starts);
}

/// Retrieves the staking end time
pub fn staking_ends() -> u64 {
    get_key(STAKING_ENDS).unwrap_or_revert()
}

/// Sets the staking end time
pub fn set_staking_ends(staking_ends: u64) {
    set_key(STAKING_ENDS, staking_ends);
}

/// Retrieves the withdrawal start time
pub fn withdraw_starts() -> u64 {
    get_key(WITHDRAW_STARTS).unwrap_or_default()
}

/// Sets the withdrawal start time
pub fn set_withdraw_starts(withdraw_starts: u64) {
    set_key(WITHDRAW_STARTS, withdraw_starts);
}

/// Retrieves the withdrawal end time
pub fn withdraw_ends() -> u64 {
    get_key(WITHDRAW_ENDS).unwrap_or_default()
}

/// Sets the withdrawal end time
pub fn set_withdraw_ends(withdraw_ends: u64) {
    set_key(WITHDRAW_ENDS, withdraw_ends);
}

/// Retrieves the total staking amount
pub fn staking_total() -> U256 {
    get_key(STAKING_TOTAL).unwrap_or_default()
}

/// Sets the total staking amount
pub fn set_staking_total(staking_total: U256) {
    set_key(STAKING_TOTAL, staking_total);
}

/// Retrieves the total reward amount
pub fn total_reward() -> U256 {
    get_key(TOTAL_REWARD).unwrap_or_default()
}

/// Sets the total reward amount
pub fn set_total_reward(total_reward: U256) {
    set_key(TOTAL_REWARD, total_reward);
}

/// Retrieves the early withdrawal reward amount
pub fn early_withdraw_reward() -> U256 {
    get_key(EARLY_WITHDRAW_REWARD).unwrap_or_default()
}

/// Sets the early withdrawal reward amount
pub fn set_early_withdraw_reward(early_withdraw_reward: U256) {
    set_key(EARLY_WITHDRAW_REWARD, early_withdraw_reward);
}

/// Retrieves the total staked amount
pub fn staked_total() -> U256 {
    get_key(STAKED_TOTAL).unwrap_or_default()
}

/// Sets staked total
pub fn set_staked_total(staked_total: U256) {
    set_key(STAKED_TOTAL, staked_total);
}

/// Retrieves the reward balance
pub fn reward_balance() -> U256 {
    get_key(REWARD_BALANCE).unwrap_or_default()
}

/// Sets the reward balance
pub fn set_reward_balance(reward_balance: U256) {
    set_key(REWARD_BALANCE, reward_balance);
}

/// Retrieves the staked balance
pub fn staked_balance() -> U256 {
    get_key(STAKED_BALANCE).unwrap_or_default()
}

/// Sets the staked balance
pub fn set_staked_balance(staked_balance: U256) {
    set_key(STAKED_BALANCE, staked_balance);
}

/// Retrieves the contract package hash
pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert();
    let package_hash: Option<ContractPackageHash> = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Some(*contract_package_hash),
        _ => None,
    };
    package_hash.unwrap_or_revert()
}

/// Emits a StakingContractEvent
pub fn emit(event: &StakingContractEvent) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match event {
        StakingContractEvent::Stake {
            token_address,
            staker_address,
            requested_amount,
            staked_amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert(EVENT_TYPE, STAKE.to_string());
            param.insert(TOKEN_ADDRESS, token_address.to_string());
            param.insert(
                STAKER_ADDRESS,
                TryInto::<String>::try_into(*staker_address).unwrap(),
            );
            param.insert(REQUESTED_AMOUNT, requested_amount.to_string());
            param.insert(STAKED_AMOUNT, staked_amount.to_string());
            events.push(param);
        }
        StakingContractEvent::PaidOut {
            token_address,
            staker_address,
            amount,
            reward,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert(EVENT_TYPE, PAID_OUT.to_string());
            param.insert(TOKEN_ADDRESS, token_address.to_string());
            param.insert(
                STAKER_ADDRESS,
                TryInto::<String>::try_into(*staker_address).unwrap(),
            );
            param.insert(AMOUNT, amount.to_string());
            param.insert(REWARD, reward.to_string());
            events.push(param);
        }
        StakingContractEvent::AddReward {
            reward_amount,
            withdrawable_amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert(EVENT_TYPE, ADD_REWARD.to_string());
            param.insert(REWARD_AMOUNT, reward_amount.to_string());
            param.insert(WITHDRAWABLE_AMOUNT, withdrawable_amount.to_string());
            events.push(param);
        }
        StakingContractEvent::Refunded {
            token_address,
            staker_address,
            amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert(EVENT_TYPE, REFUNDED.to_string());
            param.insert(TOKEN_ADDRESS, token_address.to_string());
            param.insert(
                STAKER_ADDRESS,
                TryInto::<String>::try_into(*staker_address).unwrap(),
            );
            param.insert(AMOUNT, amount.to_string());
            events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
