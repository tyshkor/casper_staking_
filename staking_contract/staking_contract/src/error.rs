use casper_types::ApiError;

#[derive(Debug)]
#[repr(u16)]
pub enum Error {
    PermissionDenied = 1,
    WrongArguments = 2,
    NotRequiredStake = 3,
    AfterBadTiming = 4,
    BeforeBadTiming = 5,
    InvalidContext = 6,
    NegativeReward = 7,
    NegativeWithdrawableReward = 8,
    NegativeAmount = 9,
    MissingContractPackageHash = 10,
    InvalidContractPackageHash = 11,
    InvalidContractHash = 12,
    NeitherAccountHashNorNeitherContractPackageHash = 13,
    NotAStaker = 14,
    ImmediateCallerAddressFail = 15,
    NotStakingContractPackageHash = 16,
    StakingEndsBeforeStakingStarts = 17,
    WithdrawStartsStakingEnds = 18,
    WithdrawEndsWithdrawStarts = 19,
    StakingStartsNow = 20,
    CheckedSub = 21,
    WrongERC20Token = 22,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
