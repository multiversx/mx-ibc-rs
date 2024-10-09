// Clippy is stupid. Thinks "entry_point" is unused
#![allow(unused_imports)]
use common_modules2::require;
use cosmwasm_std::{entry_point, Addr, DepsMut, Env, MessageInfo, Response, StdResult, Storage};
use cw_storage_plus::Item;
use host_config::{bind_port, register_client, set_expected_time_per_block};
use msg::{ExecuteMsg, InstantiateMsg};

pub mod host_config;
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

fn require_owner_caller(storage: &dyn Storage, caller: &Addr) -> StdResult<()> {
    let owner = OWNER.load(storage)?;
    require!(owner == caller, "Only owner may call this endpoint");

    Ok(())
}

/*
pub fn add_members(
        deps: DepsMut,
        info: MessageInfo,
        admins: Vec<String>,
    ) -> StdResult<Response>

pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response>
*/

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
Queries:

TODO: Maybe I can return an Enum instead?

#[derive(Serialize, Deserialize)]
struct QueryResp {
    message: String,
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    let resp = QueryResp {
        message: "Hello World".to_owned(),
    };

    to_json_binary(&resp)
}
*/

/*
Some other derive:

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GreetResp)]
    Greet {},
    #[returns(AdminsListResp)]
    AdminsList {},
}

*/

/*
In bin/schema.rs:

use contract::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg
    }
}
*/

/*
How to import this:
[dependencies]
my_contract = { version = "0.1", features = ["library"] }
*/

// DON'T USE usize in serializable types!!!
