use common_modules2::{require, utils::IsEmptyStorageMap};
use common_types2::{ChannelId, PortId};
use cosmwasm_std::{Addr, StdResult, Storage};

use crate::storage::host_storage_keys::{CHANNEL_CAPABILITIES, PORT_CAPABILITIES};

pub fn claim_port_capability(
    storage: &mut dyn Storage,
    port_id: &PortId,
    address: &Addr,
) -> StdResult<()> {
    require!(
        PORT_CAPABILITIES.is_empty_at_key(storage, port_id),
        "Port already claimed"
    );

    PORT_CAPABILITIES.save(storage, port_id, address)
}

pub fn claim_channel_capability(
    storage: &mut dyn Storage,
    port_id: &PortId,
    channel_id: &ChannelId,
    address: &Addr,
) -> StdResult<()> {
    require!(
        CHANNEL_CAPABILITIES.is_empty_at_key(storage, &(port_id, channel_id)),
        "Channel already claimed"
    );

    CHANNEL_CAPABILITIES.save(storage, &(port_id, channel_id), address)
}

pub fn authenticate_channel_capability(
    storage: &dyn Storage,
    port_id: &PortId,
    channel_id: &ChannelId,
    user: &Addr,
) -> StdResult<()> {
    require!(
        !CHANNEL_CAPABILITIES.is_empty_at_key(storage, &(port_id, channel_id)),
        "Channel not claimed"
    );

    let stored_addr = CHANNEL_CAPABILITIES.load(storage, &(port_id, channel_id))?;
    require!(stored_addr == user, "Not allowed to use this port");

    Ok(())
}

pub fn lookup_module_by_port(storage: &dyn Storage, port_id: &PortId) -> StdResult<Addr> {
    require!(
        !PORT_CAPABILITIES.is_empty_at_key(storage, port_id),
        "Port not found"
    );

    PORT_CAPABILITIES.load(storage, port_id)
}

pub fn lookup_module_by_channel(
    storage: &dyn Storage,
    port_id: &PortId,
    channel_id: &ChannelId,
) -> StdResult<Addr> {
    require!(
        !CHANNEL_CAPABILITIES.is_empty_at_key(storage, &(port_id, channel_id)),
        "Channel not found"
    );

    CHANNEL_CAPABILITIES.load(storage, &(port_id, channel_id))
}
