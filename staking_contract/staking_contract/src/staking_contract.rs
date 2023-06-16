use crate::detail;
use crate::error::Error;
use crate::modifiers;
use crate::{
    address::Address,
    data::{self, StakedTokens},
    event::StakingContractEvent,
};
use alloc::string::String;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, BlockTime, ContractPackageHash, Key, RuntimeArgs, U256};
use contract_utils::{ContractContext, ContractStorage};

const STACKING_CONTRACT_PACKAGE_HASH: &str = "stacking_contract_package_hash";
const ERC20_CONTRACT_PACKAGE_HASH: &str = "erc20_contract_package_hash";

const ENTRY_POINT_TRANSFER: &str = "transfer";

// This code defines a trait for the staking contract.
#[allow(clippy::too_many_arguments)]
pub trait CEP20STK<Storage: ContractStorage>: ContractContext<Storage> {
    // Initializes the staking contract.
    #[allow(clippy::too_many_arguments)]
    fn init(
        &mut self,
        name: String,
        address: String,
        staking_starts: u64,
        staking_ends: u64,
        withdraw_starts: u64,
        withdraw_ends: u64,
        staking_total: U256,
    ) -> Result<(), Error> {
        if staking_ends < staking_starts {
            return Err(Error::StakingEndsBeforeStakingStarts);
        }
        if withdraw_starts < staking_ends {
            return Err(Error::WithdrawStartsStakingEnds);
        }
        if withdraw_ends < withdraw_starts {
            return Err(Error::WithdrawEndsWithdrawStarts);
        }
        if staking_starts < u64::from(runtime::get_blocktime()) {
            return Err(Error::StakingStartsNow);
        }
        if staking_ends != withdraw_starts {
            return Err(Error::GapBetweenStakingEndsWithdrawStarts);
        }
        data::set_name(name);
        data::set_address(address);
        data::set_staking_starts(staking_starts);
        data::set_staking_ends(staking_ends);
        data::set_withdraw_starts(withdraw_starts);
        data::set_withdraw_ends(withdraw_ends);
        data::set_staking_total(staking_total);
        // Initialize the staked tokens map.
        StakedTokens::init();
        Ok(())
    }

    // Returns the contract name.
    fn name(&self) -> String {
        data::name()
    }

    // Returns the contract address.
    fn address(&self) -> String {
        data::address()
    }

    // Returns the staking start time.
    fn staking_starts(&self) -> u64 {
        data::staking_starts()
    }

    // Returns the staking end time.
    fn staking_ends(&self) -> u64 {
        data::staking_ends()
    }

    // Returns the withdraw start time.
    fn withdraw_starts(&self) -> u64 {
        data::withdraw_starts()
    }

    // Returns the withdraw end time.
    fn withdraw_ends(&self) -> u64 {
        data::withdraw_ends()
    }

    // Returns the total staking amount.
    fn staking_total(&self) -> U256 {
        data::staking_total()
    }

    // Sets the total staking amount.
    fn set_staking_total(&self, staking_total: U256) {
        data::set_staking_total(staking_total)
    }

    // Returns the reward balance.
    fn reward_balance(&self) -> U256 {
        data::reward_balance()
    }

    // Sets the staked balance.
    fn set_reward_balance(&self, reward_balance: U256) {
        data::set_reward_balance(reward_balance)
    }

    // Returns the staked balance.
    fn staked_balance(&self) -> U256 {
        data::staked_balance()
    }

    // Sets the staked balance.
    fn set_staked_balance(&self, staked_balance: U256) {
        data::set_staked_balance(staked_balance)
    }

    // Returns the total reward.
    fn total_reward(&self) -> U256 {
        data::total_reward()
    }

    // Sets the total reward.
    fn set_total_reward(&self, total_reward: U256) {
        data::set_total_reward(total_reward)
    }

    // Returns the early withdraw reward.
    fn early_withdraw_reward(&self) -> U256 {
        data::early_withdraw_reward()
    }

    // Sets the early withdraw reward.
    fn set_early_withdraw_reward(&self, early_withdraw_reward: U256) {
        data::set_early_withdraw_reward(early_withdraw_reward)
    }

    fn staked_total(&self) -> U256 {
        data::staked_total()
    }

    fn set_staked_total(&self, staked_total: U256) {
        data::set_staked_total(staked_total)
    }

    /// Returns the amount of tokens that have been staked by the given staker.
    fn amount_staked(&self, staker: Key) -> Result<U256, Error> {
        StakedTokens::instance()
            .get_amount_staked_by_address(&staker)
            .ok_or(Error::NotAStaker)
    }

    /// Stakes the given amount of tokens.
    fn stake(&mut self, amount: U256) -> Result<U256, Error> {
        modifiers::positive(amount)?;
        modifiers::after(self.staking_starts())?;
        modifiers::before(self.staking_ends())?;
        // check for has enough tokens

        let token_address = self.address();

        let stakers_dict = StakedTokens::instance();
        let staker_address = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerAddressFail);

        let mut remaining_token = amount;

        if let Some(diff) = self.staking_total().checked_sub(amount) {
            if remaining_token > diff {
                remaining_token = diff;
            }
        }

        if remaining_token <= U256::from(0u64) {
            return Err(Error::NotRequiredStake);
        }

        if (remaining_token + self.staked_total()) > self.staking_total() {
            return Err(Error::NotRequiredStake);
        }

        self.pay_me(staker_address, remaining_token);

        self.emit(StakingContractEvent::Stake {
            token_address,
            staker_address,
            requested_amount: amount,
            staked_amount: remaining_token,
        });

        if remaining_token < amount {
            let refund = amount - remaining_token;
            self.pay_to(staker_address, staker_address, refund);
        }

        self.set_staked_total(self.staked_total() + remaining_token);
        self.set_staked_balance(self.staked_balance() + remaining_token);
        stakers_dict.add_stake(&Key::from(staker_address), &remaining_token);
        Ok(amount)
    }

    /// Withdraws the given amount of tokens.
    fn withdraw(&mut self, amount: U256) -> Result<U256, Error> {
        modifiers::positive(amount)?;
        modifiers::after(self.withdraw_starts())?;

        let stakers_dict = StakedTokens::instance();
        let caller_address = detail::get_immediate_caller_address()?;

        if amount
            > stakers_dict
                .get_amount_staked_by_address(&Key::from(caller_address))
                .unwrap()
        {
            return Err(Error::NotRequiredStake);
        }

        // different flows depending on when staking ends
        if runtime::get_blocktime() < BlockTime::new(self.withdraw_ends()) {
            self.withdraw_early(amount, caller_address)
        } else {
            self.withdraw_after_close(amount, caller_address)
        }
    }

    /// Withdraws the given amount of tokens early.
    fn withdraw_early(&mut self, amount: U256, caller_address: Address) -> Result<U256, Error> {
        let staker_address = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerAddressFail);
        let token_address = self.address();

        let denom = U256::from(
            self.withdraw_ends()
                .checked_sub(self.staking_ends())
                .ok_or(Error::CheckedSub)?,
        ) * self.staked_total();

        let reward: U256 = U256::from(
            u64::from(runtime::get_blocktime())
                .checked_sub(self.staking_ends())
                .ok_or(Error::CheckedSub)?,
        ) * self.early_withdraw_reward()
            * amount
            / denom;

        let pay_out = amount + reward;

        self.set_reward_balance(
            self.reward_balance()
                .checked_sub(reward)
                .ok_or(Error::CheckedSub)?,
        );
        self.set_staked_balance(
            self.staked_balance()
                .checked_sub(amount)
                .ok_or(Error::CheckedSub)?,
        );
        let stakers_dict = StakedTokens::instance();
        stakers_dict.withdraw_stake(&Key::from(caller_address), &amount)?;
        // pay the tokens
        self.pay_direct(caller_address, pay_out)?;
        // emit `PaidOut` event
        self.emit(StakingContractEvent::PaidOut {
            staker_address,
            token_address,
            amount,
            reward,
        });
        Ok(amount)
    }

    /// Withdraws the given amount of tokens after the staking period has ended.
    fn withdraw_after_close(
        &mut self,
        amount: U256,
        caller_address: Address,
    ) -> Result<U256, Error> {
        let staker_address = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerAddressFail);
        let token_address = self.address();

        let reward = self.reward_balance() * amount / self.staked_balance();
        let pay_out = amount + reward;
        let stakers_dict = StakedTokens::instance();
        // mutate stakers_dict accordingly to the situation
        stakers_dict.withdraw_stake(&Key::from(caller_address), &amount)?;
        self.pay_direct(caller_address, pay_out)?;
        // emit `PaidOut` event
        self.emit(StakingContractEvent::PaidOut {
            staker_address,
            token_address,
            amount,
            reward,
        });
        Ok(amount)
    }

    /// Adds the given amount of reward tokens.
    fn add_reward(
        &mut self,
        reward_amount: U256,
        withdrawable_amount: U256,
    ) -> Result<U256, Error> {
        modifiers::before(self.withdraw_starts())?;

        // reward_amount has to be positive
        if reward_amount <= U256::from(0u64) {
            return Err(Error::NegativeReward);
        }

        if withdrawable_amount < U256::from(0u64) {
            return Err(Error::NegativeWithdrawableReward);
        }

        if withdrawable_amount > reward_amount {
            return Err(Error::NegativeWithdrawableReward);
        }
        self.pay_me(detail::get_immediate_caller_address()?, reward_amount);

        // calculate new total reward
        let current_total_reward = self.total_reward() + reward_amount;

        self.set_total_reward(current_total_reward);
        self.set_reward_balance(current_total_reward);
        self.set_early_withdraw_reward(self.early_withdraw_reward() + withdrawable_amount);

        Ok(reward_amount)
    }

    /// Returns the reward that the given staker is entitled to.
    fn staker_reward(&mut self, staker_address: Key) -> Result<U256, Error> {
        let amount = self.amount_staked(staker_address)?;
        let reward: U256 = if runtime::get_blocktime() < BlockTime::new(self.staking_ends()) {
            let denom = U256::from(
                self.withdraw_ends()
                    .checked_sub(self.staking_ends())
                    .ok_or(Error::CheckedSub)?,
            ) * self.staked_total();

            U256::from(
                u64::from(runtime::get_blocktime())
                    .checked_sub(self.staking_ends())
                    .ok_or(Error::CheckedSub)?,
            ) * amount
                / denom
        } else {
            self.reward_balance() * amount / self.staked_balance()
        };

        Ok(reward)
    }

    /// Pays the given amount of tokens directly to the recipient.
    fn pay_direct(&self, recipient: Address, amount: U256) -> Result<(), Error> {
        modifiers::positive(amount)?;
        let erc20_contract_package_hash = self.erc20_contract_package_hash();

        let args = runtime_args! {
            "recipient" => recipient,
            "amount" => amount,
        };
        runtime::call_versioned_contract::<()>(
            erc20_contract_package_hash,
            None,
            ENTRY_POINT_TRANSFER,
            args,
        );
        Ok(())
    }

    /// Pays the given amount of tokens to the recipient, transferring them from the given allower.
    fn pay_to(&self, allower: Address, recipient: Address, amount: U256) {
        let erc20_contract_package_hash = self.erc20_contract_package_hash();
        let args = runtime_args! {
            "owner" => allower,
            "recipient" => recipient,
            "amount" => amount
        };
        runtime::call_versioned_contract::<()>(
            erc20_contract_package_hash,
            None,
            "transfer_from",
            args,
        );
    }

    /// Pays the given amount of tokens to the staking contract, transferring them from the given allower.
    fn pay_me(&self, payer: Address, amount: U256) {
        #[allow(clippy::redundant_closure)]
        let stacking_contract_package_hash = runtime::get_key(STACKING_CONTRACT_PACKAGE_HASH)
            .unwrap_or_revert_with(Error::MissingContractPackageHash)
            .into_hash()
            .map(|hash_address| ContractPackageHash::new(hash_address))
            .unwrap_or_revert_with(Error::InvalidContractPackageHash);
        self.pay_to(
            payer,
            crate::address::Address::ContractPackage(stacking_contract_package_hash),
            amount,
        )
    }

    /// Emits the events
    fn emit(&mut self, event: StakingContractEvent) {
        data::emit(&event);
    }

    /// Returns `ContractPackageHash` of the ERC-20 type token that is staked
    fn erc20_contract_package_hash(&self) -> ContractPackageHash {
        #[allow(clippy::redundant_closure)]
        runtime::get_key(ERC20_CONTRACT_PACKAGE_HASH)
            .unwrap_or_revert_with(Error::MissingContractPackageHash)
            .into_hash()
            .map(|hash_address| ContractPackageHash::new(hash_address))
            .unwrap_or_revert_with(Error::InvalidContractHash)
    }
}
