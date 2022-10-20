use crate::{
    address::Address,
    data::{self, StakedTokens},
    event::CEP47Event,
};
use alloc::{
    fmt::format,
    string::{String, ToString},
};
use casper_types::{account::AccountHash, RuntimeArgs};
use casper_types::{runtime_args, ApiError, BlockTime, ContractPackageHash, Key, U256};
use contract_utils::{ContractContext, ContractStorage};
// use core::convert::TryInto;
use crate::detail;
use crate::modifiers;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::ContractHash;

#[derive(Debug)]
#[repr(u16)]
pub enum Error {
    PermissionDenied = 1,
    WrongArguments = 2,
    NotRequiredStake = 3,
    BadTiming = 4,
    InvalidContext = 5,
    NegativeReward = 6,
    NegativeWithdrawableReward = 7,
    NegativeAmount = 8,
    MissingContractPackageHash = 9,
    InvalidContractPackageHash = 10,
    InvalidContractHash = 11,
    WithdrawCheckErrorEarly = 12,
    WithdrawCheckError = 13,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait CEP20STK<Storage: ContractStorage>: ContractContext<Storage> {
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

    fn amount_staked(&self, staker: Key) -> U256 {
        StakedTokens::instance()
            .get_amount_staked_by_address(&staker)
            .unwrap()
    }

    fn stake(&mut self, amount: U256) -> Result<U256, Error> {
        // modifiers::positive(amount)?;
        // modifiers::after(self.staking_starts())?;
        // modifiers::before(self.staking_ends())?;

        let token_address = self.address();

        let stakers_dict = StakedTokens::instance();
        let stacker_addr =
            detail::get_immediate_caller_address().unwrap_or_revert_with(Error::NegativeReward);

        let mut remaining_token = amount;

        // ?
        if remaining_token > (self.staking_total() - remaining_token) {
            remaining_token = self.staking_total() - remaining_token;
        }

        if remaining_token <= U256::from(0u64) {
            return Err(Error::NotRequiredStake);
        }

        if (remaining_token + self.staked_total()) > self.staking_total() {
            return Err(Error::NotRequiredStake);
        }

        self.pay_me(
            detail::get_immediate_caller_address().unwrap_or_revert_with(Error::NegativeReward),
            remaining_token,
        );

        // runtime::put_key("answer", remaining_token.into())

        self.emit(CEP47Event::Stake { amount });

        if remaining_token < amount {
            let refund = amount - remaining_token;
            self.pay_direct(stacker_addr, refund)
                .unwrap_or_revert_with(Error::NegativeAmount);
        }

        self.set_staking_total(self.staking_total() + remaining_token);
        stakers_dict.add_stake(&Key::from(stacker_addr), &remaining_token);
        Ok(amount)
    }

    fn withdraw(&mut self, amount: U256) -> Result<U256, Error> {
        modifiers::positive(amount)?;
        // modifiers::after(self.staking_starts())?;

        let stakers_dict = StakedTokens::instance();
        let caller_address = detail::get_immediate_caller_address()?;

        // let message = format(format_args!("stakers_dict.get_amount_staked_by_address(&Key::from(caller_address)).unwrap() {:?}", stakers_dict.get_amount_staked_by_address(&Key::from(caller_address)).unwrap()));

        // runtime::put_key(
        //     &alloc::format!("{}_withdraw", message),
        //     caller_address.into(),
        // );

        // let message_2 = format(format_args!("withdraw_ends is {:?}", self.withdraw_ends()));

        // runtime::put_key(
        //     &alloc::format!("{}_withdraw", message_2),
        //     caller_address.into(),
        // );

        if amount
            > stakers_dict
                .get_amount_staked_by_address(&Key::from(caller_address))
                .unwrap()
        {
            return Err(Error::NotRequiredStake);
        }

        if runtime::get_blocktime() < BlockTime::new(self.staking_ends()) {
            self.withdraw_early(amount, caller_address)
            // Err(Error::WithdrawCheckErrorEarly)
        } else {
            // self.withdraw_after_close(amount, caller_address)
            Err(Error::WithdrawCheckError)
        }
    }

    fn withdraw_early(&mut self, amount: U256, caller_address: Address) -> Result<U256, Error> {
        // let message = format(format_args!("withdraw_early amount {:?}", amount));

        // runtime::put_key(
        //     &alloc::format!("{}_withdraw", message),
        //     caller_address.into(),
        // );

        let denom = U256::from(self.withdraw_ends() - self.staking_ends()) * self.staking_total();

        let reward: U256 =
            U256::from(u64::from(runtime::get_blocktime()) - self.staking_ends()) * amount / denom;

        let pay_out = amount + reward;

        self.set_reward_balance(self.reward_balance() - reward);
        // self.set_staked_balance(self.staked_balance() - amount);
        let stakers_dict = StakedTokens::instance();
        stakers_dict.withdraw_stake(&Key::from(caller_address), &amount);
        self.pay_direct(caller_address, pay_out)?;
        self.emit(CEP47Event::Withdraw { amount });
        Ok(amount)
    }

    fn withdraw_after_close(
        &mut self,
        amount: U256,
        caller_address: Address,
    ) -> Result<U256, Error> {
        let message = format(format_args!("withdraw_after_close amount {:?}", amount));

        runtime::put_key(
            &alloc::format!("{}_withdraw", message),
            caller_address.into(),
        );

        let reward = self.reward_balance() * amount / self.staked_balance();
        let pay_out = amount + reward;
        let stakers_dict = StakedTokens::instance();
        stakers_dict.withdraw_stake(&Key::from(caller_address), &amount);
        self.pay_direct(caller_address, pay_out)?;
        self.emit(CEP47Event::Withdraw { amount });
        Ok(amount)
    }

    fn add_reward(
        &mut self,
        reward_amount: U256,
        withdrawable_amount: U256,
    ) -> Result<U256, Error> {
        // modifiers::before(self.staking_ends())?;

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

    fn pay_direct(&self, recipient: Address, amount: U256) -> Result<(), Error> {
        // modifiers::positive(amount)?;
        // let (contract_hash, _, _) = self.contract_metadata();
        let erc20_contract_hash = self.erc20_metadata();

        let args = runtime_args! {
            "recipient" => recipient,
            "amount" => amount,
            // "amount" => U256::from(1u64),
        };
        runtime::call_contract::<()>(erc20_contract_hash, "transfer", args);
        Ok(())
    }

    fn pay_to(&self, allower: Address, recipient: Address, amount: U256) {
        // let (contract_hash, _, _) = self.contract_metadata();
        let erc20_contract_hash = self.erc20_metadata();
        let args = runtime_args! {
            "owner" => allower,
            "recipient" => recipient,
            "amount" => amount
        };
        runtime::call_contract::<()>(erc20_contract_hash, "transfer_from", args);
    }

    fn pay_me(&self, payer: Address, amount: U256) {
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

    fn approve(&self, owner: Address, amount: U256) {
        let (contract_hash, contract_package_hash, address) = self.contract_metadata();
        let args = runtime_args! {
            "spender" => crate::address::Address::ContractHash(contract_hash),
            "amount" => amount
        };
        runtime::call_contract::<U256>(contract_hash, "approve", args);
    }

    fn emit(&mut self, event: CEP47Event) {
        data::emit(&event);
    }

    fn contract_metadata(&self) -> (ContractHash, ContractPackageHash, String) {
        let lower_contracthash =
            "contract-7d4d39dc4d3c017e3153c221279dd5343b7eada42b97e8a554b9bc0d7cf4f602"
                .to_lowercase();
        let contract_hash = ContractHash::from_formatted_str(&lower_contracthash).unwrap();

        let lower_contractpackagehash =
            "hashc7cc98beb3e4f0940c9d793c96fc78806b381c055c50d1729ffc1d014be89627".to_lowercase();
        let contract_package_hash =
            ContractPackageHash::from_formatted_str(&lower_contractpackagehash).unwrap();

        let address_str =
            "account-hash-8c07f894322d86705f9804d682a9ed6c9cd4be7a8fc6889d20b446e1d852fa8c";

        // let account_hash = AccountHash::from_formatted_str(address_str).unwrap();

        // let address = crate::address::Address::Account(account_hash);

        let address =
            "017fbbccf39a639a1a5f469e3fb210d9f355b532bd786f945409f0fc9a8c6313b1".to_string();
        (contract_hash, contract_package_hash, address)
    }

    fn erc20_metadata(&self) -> ContractHash {
        runtime::get_key("erc20_contract_hash")
            .unwrap_or_revert_with(Error::MissingContractPackageHash)
            .into_hash()
            .map(|hash_address| ContractHash::new(hash_address))
            .unwrap_or_revert_with(Error::InvalidContractHash)
    }

    fn approve1(&mut self, spender: Key) -> Result<(), Error> {
        let caller = self.get_caller();
        Ok(())
    }
}
