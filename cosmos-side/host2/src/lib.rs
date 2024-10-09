// Clippy is stupid. Thinks "entry_point" is unused
#![allow(unused_imports)]
use common_modules2::require;
use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Storage,
};
use cw_storage_plus::Item;
use host_config::{bind_port, register_client, set_expected_time_per_block};
use host_views::{check_and_get_client, get_commitment, get_commitment_prefix, get_host_timestamp};
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

pub mod host_config;
pub mod host_views;
pub mod module_manager;
pub mod msg;
pub mod storage;

pub const OWNER: Item<Addr> = Item::new("owner");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    let result = match msg {
        ExecuteMsg::SetExpectedTimePerBlock { exp_time_per_block } => {
            require_owner_caller(deps.storage, &info.sender)?;

            set_expected_time_per_block(deps.storage, exp_time_per_block)
        }
        ExecuteMsg::RegisterClient {
            client_type,
            client,
        } => {
            require_owner_caller(deps.storage, &info.sender)?;

            register_client(deps.storage, client_type, client)
        }
        ExecuteMsg::BindPort { port_id, module } => {
            require_owner_caller(deps.storage, &info.sender)?;

            bind_port(deps.storage, &env, port_id, module)
        }
    };

    match result {
        Ok(()) => Ok(Response::default()),
        Err(err) => Err(err),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let query_result = match msg {
        QueryMsg::GetCommitment { commitment_hash } => {
            let commitment = get_commitment(deps.storage, &commitment_hash)?;
            to_json_binary(&commitment)?
        }
        QueryMsg::GetHostTimestamp {} => {
            let host_timestamp = get_host_timestamp(&env)?;
            to_json_binary(&host_timestamp)?
        }
        QueryMsg::GetCommitmentPrefix {} => {
            let prefix = get_commitment_prefix();
            to_json_binary(&prefix)?
        }
        QueryMsg::CheckAndGetClient { client_id } => {
            let client = check_and_get_client(deps.storage, &client_id)?;
            to_json_binary(&client)?
        }
    };

    Ok(query_result)
}

fn require_owner_caller(storage: &dyn Storage, caller: &Addr) -> StdResult<()> {
    let owner = OWNER.load(storage)?;
    require!(owner == caller, "Only owner may call this endpoint");

    Ok(())
}

/*
Events:
let events = admins
            .iter()
            .map(|admin| Event::new("admin_added").add_attribute("addr", admin));

        let resp = Response::new()
            .add_events(events)
            .add_attribute("action", "add_members")
            .add_attribute("added_count", admins.len().to_string());
*/

/*
How to import this:
[dependencies]
my_contract = { version = "0.1", features = ["library"] }
*/

// DON'T USE usize in serializable types!!!
