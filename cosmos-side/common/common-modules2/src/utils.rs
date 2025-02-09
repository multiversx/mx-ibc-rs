use common_types2::{channel_types::channel, UnixTimestamp};
use common_types2::{Hash, HASH_LENGTH};
use cosmwasm_std::{Addr, Env, StdError, StdResult, Storage};
use cw_storage_plus::{Item, Map, PrimaryKey};
use keccak_hash::keccak_buffer;
use serde::de::DeserializeOwned;
use serde::Serialize;

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

pub fn vec_u8_to_str(input_vec: Vec<u8>) -> String {
    match String::from_utf8(input_vec) {
        Ok(v) => v,
        Err(_) => panic!("Invalid UTF-8 sequence"),
    }
}

pub fn keccak256(input_data: &Vec<u8>) -> Hash {
    keccak_buffer(&mut input_data.as_slice()).unwrap().into()
}

// is_empty impls

pub trait IsEmptyStorageItem {
    fn is_empty(&self, storage: &dyn Storage) -> bool;
}

pub trait IsEmptyStorageMap<'a, T: PrimaryKey<'a>> {
    fn is_empty_at_key(&self, storage: &dyn Storage, key: &'a T) -> bool;
}

impl<T> IsEmptyStorageItem for Item<T>
where
    T: Serialize + DeserializeOwned,
{
    fn is_empty(&self, storage: &dyn Storage) -> bool {
        let may_load_result = self.may_load(storage);

        matches!(may_load_result, Ok(None))
    }
}

impl<'a, T, U> IsEmptyStorageMap<'a, T> for Map<&'a T, U>
where
    T: PrimaryKey<'a>,
    U: Serialize + DeserializeOwned,
{
    fn is_empty_at_key(&self, storage: &dyn Storage, key: &'a T) -> bool {
        let may_load_result = self.may_load(storage, key);

        matches!(may_load_result, Ok(None))
    }
}
