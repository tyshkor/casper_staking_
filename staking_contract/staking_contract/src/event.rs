// This code defines an enum for the events that can be emitted by the staking contract.
use crate::address::Address;
use alloc::string::String;
use casper_types::U256;

/// An enum for the events that can be emitted by the staking contract.
pub enum StakingContractEvent {
    /// Event emitted when a user stakes tokens.
    Stake {
        /// The address of the token contract.
        token_address: String,
        /// The address of the staker.
        staker_address: Address,
        /// The amount of tokens requested to be staked.
        requested_amount: U256,
        /// The amount of tokens that were actually staked.
        staked_amount: U256,
    },
    /// Event emitted when a user is paid out rewards.
    PaidOut {
        /// The address of the token contract.
        token_address: String,
        /// The address of the staker.
        staker_address: Address,
        /// The amount of tokens paid out.
        amount: U256,
        /// The amount of rewards paid out.
        reward: U256,
    },
    /// Event emitted when rewards are added to the staking contract.
    AddReward {
        /// The amount of rewards added.
        reward_amount: U256,
        /// The amount of rewards that are now withdrawable.
        withdrawable_amount: U256,
    },
    /// Event emitted when a user is refunded tokens.
    Refunded {
        /// The address of the token contract.
        token_address: String,
        /// The address of the staker.
        staker_address: Address,
        /// The amount of tokens refunded.
        amount: U256,
    },
}
