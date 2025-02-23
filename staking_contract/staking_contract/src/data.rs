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

const AMOUNT_STAKED_BY_ADDRESS_DICT: &str = "amount_staked_by_addresses_dict";
const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

pub const NAME: &str = "name";
pub const ADDRESS: &str = "address";
pub const STAKING_STARTS: &str = "staking_starts";
pub const STAKING_ENDS: &str = "staking_ends";
pub const WITHDRAW_STARTS: &str = "withdraw_starts";
pub const WITHDRAW_ENDS: &str = "withdraw_ends";
pub const STAKING_TOTAL: &str = "staking_total";
pub const STAKE_BALANCE: &str = "stake_balance";
pub const TOTAL_REWARD: &str = "total_reward";
pub const EARLY_WITHDRAW_REWARD: &str = "early_withdraw_reward";
pub const STAKED_TOTAL: &str = "staked_total";
pub const REWARD_BALANCE: &str = "reward_balance";
pub const STAKED_BALANCE: &str = "staked_balance";

pub struct StakedTokens {
    addresses_staked_dict: Dict,
}

impl StakedTokens {
    pub fn instance() -> StakedTokens {
        StakedTokens {
            addresses_staked_dict: Dict::instance(AMOUNT_STAKED_BY_ADDRESS_DICT),
        }
    }

    pub fn init() {
        Dict::init(AMOUNT_STAKED_BY_ADDRESS_DICT);
    }

    pub fn get_amount_staked_by_address(&self, address: &Key) -> Option<U256> {
        self.addresses_staked_dict.get(&key_to_str(address))
    }

    pub fn add_stake(&self, owner: &Key, amount: &U256) {
        let new_amount = if let Some(staked_amount) = self.get_amount_staked_by_address(owner) {
            staked_amount + amount
        } else {
            *amount
        };
        self.addresses_staked_dict
            .set(&key_to_str(owner), new_amount);
    }

    pub fn withdraw_stake(&self, owner: &Key, amount: &U256) -> Result<(), Error> {
        let staked_amount = self
            .get_amount_staked_by_address(owner)
            .ok_or(Error::NotAStaker)?;
        let new_amount = staked_amount - *amount;
        self.addresses_staked_dict
            .set(&key_to_str(owner), new_amount);
        Ok(())
    }
}

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn address() -> String {
    get_key(ADDRESS).unwrap_or_revert()
}

pub fn set_address(address: String) {
    set_key(ADDRESS, address);
}

pub fn staking_starts() -> u64 {
    get_key(STAKING_STARTS).unwrap_or_revert()
}

pub fn set_staking_starts(staking_starts: u64) {
    set_key(STAKING_STARTS, staking_starts);
}

pub fn staking_ends() -> u64 {
    get_key(STAKING_ENDS).unwrap_or_revert()
}

pub fn set_staking_ends(staking_ends: u64) {
    set_key(STAKING_ENDS, staking_ends);
}

pub fn withdraw_starts() -> u64 {
    get_key(WITHDRAW_STARTS).unwrap_or_default()
}

pub fn set_withdraw_starts(withdraw_starts: u64) {
    set_key(WITHDRAW_STARTS, withdraw_starts);
}

pub fn withdraw_ends() -> u64 {
    get_key(WITHDRAW_ENDS).unwrap_or_default()
}

pub fn set_withdraw_ends(withdraw_ends: u64) {
    set_key(WITHDRAW_STARTS, withdraw_ends);
}

pub fn staking_total() -> U256 {
    get_key(STAKING_TOTAL).unwrap_or_default()
}

pub fn set_staking_total(staking_total: U256) {
    set_key(STAKING_TOTAL, staking_total);
}

pub fn stake_balance() -> u64 {
    get_key(STAKED_BALANCE).unwrap_or_default()
}

pub fn set_stake_balance(stake_balance: u64) {
    set_key(STAKED_BALANCE, stake_balance);
}

pub fn total_reward() -> U256 {
    get_key(TOTAL_REWARD).unwrap_or_default()
}

pub fn set_total_reward(total_reward: U256) {
    set_key(TOTAL_REWARD, total_reward);
}

pub fn early_withdraw_reward() -> U256 {
    get_key(EARLY_WITHDRAW_REWARD).unwrap_or_default()
}

pub fn set_early_withdraw_reward(early_withdraw_reward: U256) {
    set_key(EARLY_WITHDRAW_REWARD, early_withdraw_reward);
}

pub fn staked_total() -> U256 {
    get_key(STAKED_TOTAL).unwrap_or_default()
}

pub fn set_staked_total(staked_total: U256) {
    set_key(STAKED_TOTAL, staked_total);
}

pub fn reward_balance() -> U256 {
    get_key(REWARD_BALANCE).unwrap_or_default()
}

pub fn set_reward_balance(reward_balance: U256) {
    set_key(REWARD_BALANCE, reward_balance);
}

pub fn staked_balance() -> U256 {
    get_key(STAKED_BALANCE).unwrap_or_default()
}

pub fn set_staked_balance(staked_balance: U256) {
    set_key(STAKED_BALANCE, staked_balance);
}

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
            param.insert("event_type", "stake".to_string());
            param.insert("token_address", token_address.to_string());
            param.insert(
                "staker_address",
                TryInto::<String>::try_into(*staker_address).unwrap(),
            );
            param.insert("requested_amount", requested_amount.to_string());
            param.insert("staked_amount", staked_amount.to_string());
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
            param.insert("token_address", token_address.to_string());
            param.insert(
                "staker_address",
                TryInto::<String>::try_into(*staker_address).unwrap(),
            );
            param.insert("amount", amount.to_string());
            param.insert("reward", reward.to_string());
            events.push(param);
        }
        StakingContractEvent::AddReward {
            reward_amount,
            withdrawable_amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "add_reward".to_string());
            param.insert("reward_amount", reward_amount.to_string());
            param.insert("withdrawable_amount", withdrawable_amount.to_string());
            events.push(param);
        }
        StakingContractEvent::Refunded {
            token_address,
            staker_address,
            amount,
        } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("token_address", token_address.to_string());
            param.insert(
                "staker_address",
                TryInto::<String>::try_into(*staker_address).unwrap(),
            );
            param.insert("amount", amount.to_string());
            events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
