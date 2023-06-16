// This code defines an enum for the errors that can occur in the staking contract.
use casper_types::ApiError;

/// An enum for the errors that can occur in the staking contract.
#[derive(Debug)]
#[repr(u16)]
pub enum Error {
    /// Permission denied.
    PermissionDenied = 1,
    /// Wrong arguments.
    WrongArguments = 2,
    /// Not required stake.
    NotRequiredStake = 3,
    /// After bad timing.
    AfterBadTiming = 4,
    /// Before bad timing.
    BeforeBadTiming = 5,
    /// Invalid context.
    InvalidContext = 6,
    /// Negative reward.
    NegativeReward = 7,
    /// Negative withdrawable reward.
    NegativeWithdrawableReward = 8,
    /// Negative amount.
    NegativeAmount = 9,
    /// Missing contract package hash.
    MissingContractPackageHash = 10,
    /// Invalid contract package hash.
    InvalidContractPackageHash = 11,
    /// Invalid contract hash.
    InvalidContractHash = 12,
    /// Withdraw check error early.
    WithdrawCheckErrorEarly = 13,
    /// Withdraw check error.
    WithdrawCheckError = 14,
    /// Neither account hash nor neither contract package hash.
    NeitherAccountHashNorNeitherContractPackageHash = 15,
    /// Not a staker.
    NotAStaker = 16,
    /// Immediate caller address fail.
    ImmediateCallerAddressFail = 17,
    /// Not staking contract package hash.
    NotStakingContractPackageHash = 18,
    StakingEndsBeforeStakingStarts = 19,
    WithdrawStartsStakingEnds = 20,
    WithdrawEndsWithdrawStarts = 21,
    StakingStartsNow = 22,
    /// Subtraction underflow
    CheckedSub = 23,
    /// Gap between staking_ends and withdraw_starts
    GapBetweenStakingEndsWithdrawStarts = 24,
}

impl From<Error> for ApiError {
    /// Converts an `Error` to an `ApiError`.
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
