#![no_std]
extern crate alloc;

pub mod address;
pub mod data;
pub mod detail;
pub mod error;
pub mod event;
pub mod modifiers;
pub mod staking_contract;

pub use contract_utils;
pub use staking_contract::CEP20STK;

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
