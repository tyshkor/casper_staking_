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

pub trait CEP20STK<Storage: ContractStorage>: ContractContext<Storage> {
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
    ) {
        data::set_name(name);
        data::set_address(address);
        data::set_staking_starts(staking_starts);
        data::set_staking_ends(staking_ends);
        data::set_withdraw_starts(withdraw_starts);
        data::set_withdraw_ends(withdraw_ends);
        data::set_staking_total(staking_total);
        StakedTokens::init();
    }

    fn name(&self) -> String {
        data::name()
    }

    fn address(&self) -> String {
        data::address()
    }

    fn staking_starts(&self) -> u64 {
        data::staking_starts()
    }

    fn staking_ends(&self) -> u64 {
        data::staking_ends()
    }

    fn withdraw_starts(&self) -> u64 {
        data::withdraw_starts()
    }

    fn withdraw_ends(&self) -> u64 {
        data::withdraw_ends()
    }

    fn staking_total(&self) -> U256 {
        data::staking_total()
    }

    fn set_staking_total(&self, staking_total: U256) {
        data::set_staking_total(staking_total)
    }

    fn reward_balance(&self) -> U256 {
        data::reward_balance()
    }

    fn set_reward_balance(&self, reward_balance: U256) {
        data::set_reward_balance(reward_balance)
    }

    fn staked_balance(&self) -> U256 {
        data::staked_balance()
    }

    fn set_staked_balance(&self, staked_balance: U256) {
        data::set_staked_balance(staked_balance)
    }

    fn total_reward(&self) -> U256 {
        data::total_reward()
    }

    fn set_total_reward(&self, total_reward: U256) {
        data::set_total_reward(total_reward)
    }

    fn early_withdraw_reward(&self) -> U256 {
        data::early_withdraw_reward()
    }

    fn set_early_withdraw_reward(&self, early_withdraw_reward: U256) {
        data::set_early_withdraw_reward(early_withdraw_reward)
    }

    fn staked_total(&self) -> U256 {
        data::staked_total()
    }

    fn set_staked_total(&self, staked_total: U256) {
        data::set_staked_total(staked_total)
    }

    fn amount_staked(&self, staker: Key) -> Result<U256, Error> {
        StakedTokens::instance()
            .get_amount_staked_by_address(&staker)
            .ok_or(Error::NotAStaker)
    }

    fn stake(
        &mut self,
        amount: U256,
        staking_contract_package_hash: String,
    ) -> Result<U256, Error> {
        modifiers::positive(amount)?;
        modifiers::after(self.staking_starts())?;
        modifiers::before(self.staking_ends())?;
        // check for has enough tokens

        let token_address = self.address();

        let stakers_dict = StakedTokens::instance();
        let staker_address = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerAddressFail);

        let mut remaining_token = amount;

        if let Some(diff) = self.staking_total().checked_sub(remaining_token) {
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

        let staking_contract_package_hash =
            ContractPackageHash::from_formatted_str(staking_contract_package_hash.as_str())
                .map_err(|_| Error::NotStakingContractPackageHash)?;

        self.pay_to(
            staker_address,
            crate::address::Address::ContractPackage(staking_contract_package_hash),
            remaining_token,
        );

        self.emit(StakingContractEvent::Stake {
            token_address,
            staker_address,
            requested_amount: amount,
            staked_amount: remaining_token,
        });

        if remaining_token < amount {
            let refund = amount - remaining_token;
            self.pay_direct(staker_address, refund)
                .unwrap_or_revert_with(Error::NegativeAmount);
        }

        self.set_staking_total(self.staking_total() + remaining_token);
        self.set_staked_balance(self.staked_balance() + remaining_token);
        stakers_dict.add_stake(&Key::from(staker_address), &remaining_token);
        Ok(amount)
    }

    fn withdraw(&mut self, amount: U256) -> Result<U256, Error> {
        modifiers::positive(amount)?;
        modifiers::after(self.staking_starts())?;

        let stakers_dict = StakedTokens::instance();
        let caller_address = detail::get_immediate_caller_address()?;

        if amount
            > stakers_dict
                .get_amount_staked_by_address(&Key::from(caller_address))
                .unwrap()
        {
            return Err(Error::NotRequiredStake);
        }

        if runtime::get_blocktime() < BlockTime::new(self.staking_ends()) {
            self.withdraw_early(amount, caller_address)
        } else {
            self.withdraw_after_close(amount, caller_address)
        }
    }

    fn withdraw_early(&mut self, amount: U256, caller_address: Address) -> Result<U256, Error> {
        let staker_address = detail::get_immediate_caller_address()
            .unwrap_or_revert_with(Error::ImmediateCallerAddressFail);
        let token_address = self.address();

        let denom = U256::from(self.withdraw_ends() - self.staking_ends()) * self.staking_total();

        let reward: U256 =
            U256::from(u64::from(runtime::get_blocktime()) - self.staking_ends()) * amount / denom;

        let pay_out = amount + reward;

        self.set_reward_balance(self.reward_balance() - reward);
        self.set_staked_balance(self.staked_balance() - amount);
        let stakers_dict = StakedTokens::instance();
        stakers_dict.withdraw_stake(&Key::from(caller_address), &amount)?;
        self.pay_direct(caller_address, pay_out)?;
        self.emit(StakingContractEvent::PaidOut {
            staker_address,
            token_address,
            amount,
            reward,
        });
        Ok(amount)
    }

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
        stakers_dict.withdraw_stake(&Key::from(caller_address), &amount)?;
        self.pay_direct(caller_address, pay_out)?;
        self.emit(StakingContractEvent::PaidOut {
            staker_address,
            token_address,
            amount,
            reward,
        });
        Ok(amount)
    }

    fn add_reward(
        &mut self,
        reward_amount: U256,
        withdrawable_amount: U256,
    ) -> Result<U256, Error> {
        modifiers::before(self.withdraw_starts())?;

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

        let current_total_reward = self.total_reward() + reward_amount;

        self.set_total_reward(current_total_reward);
        self.set_reward_balance(current_total_reward);
        self.set_early_withdraw_reward(self.early_withdraw_reward() + withdrawable_amount);

        Ok(reward_amount)
    }

    fn staker_reward(&mut self, staker_address: Key) -> Result<U256, Error> {
        let amount = self.amount_staked(staker_address)?;
        let reward: U256 = if runtime::get_blocktime() < BlockTime::new(self.staking_ends()) {
            let denom =
                U256::from(self.withdraw_ends() - self.staking_ends()) * self.staking_total();

            U256::from(u64::from(runtime::get_blocktime()) - self.staking_ends()) * amount / denom
        } else {
            self.reward_balance() * amount / self.staked_balance()
        };

        Ok(reward)
    }

    fn pay_direct(&self, recipient: Address, amount: U256) -> Result<(), Error> {
        // modifiers::positive(amount)?;
        let erc20_contract_package_hash = self.erc20_metadata();

        let args = runtime_args! {
            "recipient" => recipient,
            "amount" => amount,
        };
        runtime::call_versioned_contract::<()>(erc20_contract_package_hash, None, "transfer", args);
        Ok(())
    }

    fn pay_to(&self, allower: Address, recipient: Address, amount: U256) {
        let erc20_contract_package_hash = self.erc20_metadata();
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

    fn pay_me(&self, payer: Address, amount: U256) {
        #[allow(clippy::redundant_closure)]
        let stacking_contract_package_hash = runtime::get_key("stacking_contract_package_hash")
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

    fn emit(&mut self, event: StakingContractEvent) {
        data::emit(&event);
    }

    fn erc20_metadata(&self) -> ContractPackageHash {
        #[allow(clippy::redundant_closure)]
        runtime::get_key("erc20_contract_package_hash")
            .unwrap_or_revert_with(Error::MissingContractPackageHash)
            .into_hash()
            .map(|hash_address| ContractPackageHash::new(hash_address))
            .unwrap_or_revert_with(Error::InvalidContractHash)
    }
}
