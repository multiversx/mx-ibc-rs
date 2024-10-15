use common_modules2::{
    client_lib::is_valid_client_type,
    host_lib::is_valid_port_id,
    require,
    utils::{require_valid_address, IsEmptyStorageItem, IsEmptyStorageMap},
};
use common_types2::{ClientType, PortId, UnixTimestamp};
use cosmwasm_std::{Addr, Env, StdError, StdResult, Storage};

use crate::{
    module_manager::claim_port_capability,
    storage::{
        host_storage_keys::{CLIENT_REGISTRY, HOST_INFO},
        HostInfo,
    },
};

pub fn set_expected_time_per_block(
    storage: &mut dyn Storage,
    exp_time_per_block: UnixTimestamp,
) -> StdResult<()> {
    if !HOST_INFO.is_empty(storage) {
        let update_result: Result<_, StdError> =
            HOST_INFO.update(storage, |mut host_info: HostInfo| {
                host_info.expected_time_per_block = exp_time_per_block;

                Ok(host_info)
            });

        return match update_result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        };
    }

    let default_host_value = HostInfo {
        expected_time_per_block: exp_time_per_block,
        ..Default::default()
    };
    HOST_INFO.save(storage, &default_host_value)
}

pub fn register_client(
    storage: &mut dyn Storage,
    client_type: ClientType,
    client: Addr,
) -> StdResult<()> {
    require!(is_valid_client_type(&client_type), "Invalid client ID");
    require!(
        CLIENT_REGISTRY.is_empty_at_key(storage, &client_type),
        "Client already exists"
    );

    CLIENT_REGISTRY.save(storage, &client_type, &client)
}

pub fn bind_port(
    storage: &mut dyn Storage,
    env: &Env,
    port_id: PortId,
    module: Addr,
) -> StdResult<()> {
    require!(is_valid_port_id(&port_id), "Invalid Port ID");
    require_valid_address(&module, env)?;

    claim_port_capability(storage, &port_id, &module)
}
