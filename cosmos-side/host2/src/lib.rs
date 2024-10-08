#[clippy::allow(unused_imports)]
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::InstantiateMsg;

pub mod msg;
pub mod state;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    

    Ok(Response::default())
}

/*
Read storage:

let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();
    ADMINS.save(deps.storage, &admins?)?;
*/

// Save to storage: let admins = ADMINS.load(deps.storage)?;

/*
Execute endpoint:

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    contract::execute(deps, env, info, msg)
}
*/

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

// DON'T USE usize!!!
