use common_modules2::require;
use common_types2::{ChannelId, PortId};
use cosmwasm_std::{Addr, StdResult, Storage};

use crate::storage::host_storage_keys::{CHANNEL_CAPABILITIES, PORT_CAPABILITIES};

pub fn claim_port_capability(
    storage: &mut dyn Storage,
    port_id: &PortId,
    address: &Addr,
) -> StdResult<()> {
    let may_load_result = PORT_CAPABILITIES.may_load(storage, port_id);
    require!(matches!(may_load_result, Ok(None)), "Port already claimed");

    PORT_CAPABILITIES.save(storage, port_id, address)
}

pub fn claim_channel_capability(
    storage: &mut dyn Storage,
    port_id: &PortId,
    channel_id: &ChannelId,
    address: &Addr,
) -> StdResult<()> {
    let may_load_result = CHANNEL_CAPABILITIES.may_load(storage, (port_id, channel_id));
    require!(
        matches!(may_load_result, Ok(None)),
        "Channel already claimed"
    );

    CHANNEL_CAPABILITIES.save(storage, (port_id, channel_id), address)
}

pub fn authenticate_channel_capability(
    storage: &dyn Storage,
    port_id: &PortId,
    channel_id: &ChannelId,
    user: &Addr,
) -> StdResult<()> {
    let may_load_result = CHANNEL_CAPABILITIES.may_load(storage, (port_id, channel_id));
    require!(!matches!(may_load_result, Ok(None)), "Channel not claimed");

    let stored_addr = CHANNEL_CAPABILITIES.load(storage, (port_id, channel_id))?;
    require!(stored_addr == user, "Not allowed to use this port");

    Ok(())
}

pub fn lookup_module_by_port(storage: &dyn Storage, port_id: &PortId) -> StdResult<Addr> {
    let may_load_result = PORT_CAPABILITIES.may_load(storage, port_id);
    require!(!matches!(may_load_result, Ok(None)), "Port not found");

    PORT_CAPABILITIES.load(storage, port_id)
}

pub fn lookup_module_by_channel(
    storage: &dyn Storage,
    port_id: &PortId,
    channel_id: &ChannelId,
) -> StdResult<Addr> {
    let may_load_result = CHANNEL_CAPABILITIES.may_load(storage, (port_id, channel_id));
    require!(!matches!(may_load_result, Ok(None)), "Channel not found");

    CHANNEL_CAPABILITIES.load(storage, (port_id, channel_id))
}
