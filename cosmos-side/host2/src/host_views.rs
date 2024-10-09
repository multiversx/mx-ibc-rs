use common_modules2::utils::checked_timestamp_to_unix_mul;
use cosmwasm_std::{Addr, Env, StdResult, Storage};

use crate::storage::{host_helpers::try_get_client_info, host_storage_keys::COMMITMENTS};
use common_types2::{ClientId, Hash, UnixTimestamp};

// TODO: Add to queries

static DEFAULT_COMMITMENT_PREFIX: &[u8] = b"ibc";

pub fn get_commitment(storage: &dyn Storage, commitment_hash: &Hash) -> StdResult<Hash> {
    COMMITMENTS.load(storage, commitment_hash)
}

/// Returns the current timestamp (Unix time in nanoseconds) of the host chain.
pub fn get_host_timestamp(env: &Env) -> StdResult<UnixTimestamp> {
    let block_timestamp = env.block.height;

    checked_timestamp_to_unix_mul(block_timestamp)
}

#[inline]
pub fn get_commitment_prefix() -> Vec<u8> {
    DEFAULT_COMMITMENT_PREFIX.to_vec()
}

pub fn check_and_get_client(storage: &dyn Storage, client_id: &ClientId) -> StdResult<Addr> {
    let client_info = try_get_client_info(storage, client_id)?;

    Ok(client_info.client_impl)
}
