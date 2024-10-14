// Clippy is stupid. Thinks "entry_point" is unused
//#![allow(unused_imports)]
use cosmwasm_std::{
    entry_point, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw_storage_plus::Item;
use msg::{execute_host_endpoint, execute_host_query, ExecuteMsg, InstantiateMsg, QueryMsg};

pub mod commitment;
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    let result = execute_host_endpoint(deps.storage, &env, &info, msg);

    match result {
        Ok(()) => Ok(Response::default()),
        Err(err) => Err(err),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    execute_host_query(deps.storage, &env, msg)
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
