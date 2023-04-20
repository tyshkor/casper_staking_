use crate::address::Address;
use alloc::string::String;
use casper_types::U256;

pub enum StakingContractEvent {
    Stake {
        token_address: String,
        staker_address: Address,
        requested_amount: U256,
        staked_amount: U256,
    },
    PaidOut {
        token_address: String,
        staker_address: Address,
        amount: U256,
        reward: U256,
    },
    AddReward {
        reward_amount: U256,
        withdrawable_amount: U256,
    },
    Refunded {
        token_address: String,
        staker_address: Address,
        amount: U256,
    },
}
