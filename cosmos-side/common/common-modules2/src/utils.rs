use common_types2::{channel_types::channel, UnixTimestamp};
use common_types2::{Hash, HASH_LENGTH};
use cosmwasm_std::{Addr, Env, StdError, StdResult};

#[macro_export]
macro_rules! require {
    ($expression:expr, $($msg_tokens:tt),+  $(,)?) => {
        if (!($expression)) {
            return Err(cosmwasm_std::StdError::generic_err($($msg_tokens),+));
        }
    };
}

const NANO_SECONDS_MULT: u64 = 1_000_000_000;
const EMPTY_HASH: &[u8; HASH_LENGTH] = &[0u8; HASH_LENGTH];

pub static UNEXPECTED_CHANNEL_STATE_ERR_MSG: &str = "Unexpected channel state";

pub fn require_valid_address(address: &Addr, env: &Env) -> StdResult<()> {
    require!(address != env.contract.address, "Invalid address");

    Ok(())
}

pub fn require_state_open(state: channel::State) -> StdResult<()> {
    require!(
        matches!(state, channel::State::Open),
        UNEXPECTED_CHANNEL_STATE_ERR_MSG
    );

    Ok(())
}

pub fn checked_timestamp_to_unix_mul(timestamp: u64) -> StdResult<UnixTimestamp> {
    match timestamp.checked_mul(NANO_SECONDS_MULT) {
        Some(result) => Ok(result),
        None => std_err("Overflow!!!"),
    }
}

#[inline]
pub fn is_empty_hash(hash: &Hash) -> bool {
    hash == EMPTY_HASH
}

#[inline]
pub fn std_err<T>(err_msg: &str) -> StdResult<T> {
    Err(StdError::generic_err(err_msg))
}
