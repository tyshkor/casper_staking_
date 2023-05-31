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
    /// Bad timing.
    BadTiming = 4,
    /// Invalid context.
    InvalidContext = 5,
    /// Negative reward.
    NegativeReward = 6,
    /// Negative withdrawable reward.
    NegativeWithdrawableReward = 7,
    /// Negative amount.
    NegativeAmount = 8,
    /// Missing contract package hash.
    MissingContractPackageHash = 9,
    /// Invalid contract package hash.
    InvalidContractPackageHash = 10,
    /// Invalid contract hash.
    InvalidContractHash = 11,
    /// Withdraw check error early.
    WithdrawCheckErrorEarly = 12,
    /// Withdraw check error.
    WithdrawCheckError = 13,
    /// Neither account hash nor neither contract package hash.
    NeitherAccountHashNorNeitherContractPackageHash = 14,
    /// Not a staker.
    NotAStaker = 15,
    /// Immediate caller address fail.
    ImmediateCallerAddressFail = 16,
    /// Not staking contract package hash.
    NotStakingContractPackageHash = 17,
}

impl From<Error> for ApiError {
    /// Converts an `Error` to an `ApiError`.
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
