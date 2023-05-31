// This code provides a set of functions for checking the validity of a transaction.
use crate::error::Error;
use casper_contract::contract_api::runtime;
use casper_types::{BlockTime, U256};

/// Checks if the specified amount is positive.
///
/// # Arguments
///
/// * `amount`: The amount to check.
///
/// # Returns
///
/// A `Result`. If the amount is positive, the result will be `Ok(())`. If the amount is negative, the result will be `Err(Error::NegativeAmount)`.
pub fn positive(amount: U256) -> Result<(), Error> {
    if amount <= U256::from(0) {
        Err(Error::NegativeAmount)
    } else {
        Ok(())
    }
}

/// Checks if the current block time is after the specified event time.
///
/// # Arguments
///
/// * `event_time`: The event time to check.
///
/// # Returns
///
/// A `Result`. If the current block time is after the event time, the result will be `Ok(())`. If the current block time is before the event time, the result will be `Err(Error::BadTiming)`.
pub fn after(event_time: u64) -> Result<(), Error> {
    if runtime::get_blocktime() < BlockTime::new(event_time) {
        Err(Error::AfterBadTiming)
    } else {
        Ok(())
    }
}

/// Checks if the current block time is before the specified event time.
///
/// # Arguments
///
/// * `event_time`: The event time to check.
///
/// # Returns
///
/// A `Result`. If the current block time is before the event time, the result will be `Ok(())`. If the current block time is after the event time, the result will be `Err(Error::BadTiming)`.
pub fn before(event_time: u64) -> Result<(), Error> {
    if runtime::get_blocktime() >= BlockTime::new(event_time) {
        Err(Error::BeforeBadTiming)
    } else {
        Ok(())
    }
}
