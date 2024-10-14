use common_modules2::require;
use common_types2::{ClientId, ClientType, Hash, PortId, UnixTimestamp};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Addr, Binary, Env, MessageInfo, StdResult, Storage};

use crate::{
    host_config::{bind_port, register_client, set_expected_time_per_block},
    host_views::{check_and_get_client, get_commitment, get_commitment_prefix, get_host_timestamp},
    OWNER,
};

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetExpectedTimePerBlock {
        exp_time_per_block: UnixTimestamp,
    },
    RegisterClient {
        client_type: ClientType,
        client: Addr,
    },
    BindPort {
        port_id: PortId,
        module: Addr,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Hash)]
    GetCommitment { commitment_hash: Hash },

    #[returns(UnixTimestamp)]
    GetHostTimestamp {},

    #[returns(Vec<u8>)]
    GetCommitmentPrefix {},

    #[returns(Addr)]
    CheckAndGetClient { client_id: ClientId },
}

pub fn execute_host_endpoint(
    storage: &mut dyn Storage,
    env: &Env,
    info: &MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<()> {
    match msg {
        ExecuteMsg::SetExpectedTimePerBlock { exp_time_per_block } => {
            require_owner_caller(storage, &info.sender)?;

            set_expected_time_per_block(storage, exp_time_per_block)
        }
        ExecuteMsg::RegisterClient {
            client_type,
            client,
        } => {
            require_owner_caller(storage, &info.sender)?;

            register_client(storage, client_type, client)
        }
        ExecuteMsg::BindPort { port_id, module } => {
            require_owner_caller(storage, &info.sender)?;

            bind_port(storage, env, port_id, module)
        }
    }
}

pub fn execute_host_query(storage: &dyn Storage, env: &Env, msg: QueryMsg) -> StdResult<Binary> {
    let result = match msg {
        QueryMsg::GetCommitment { commitment_hash } => {
            let commitment = get_commitment(storage, &commitment_hash)?;
            to_json_binary(&commitment)?
        }
        QueryMsg::GetHostTimestamp {} => {
            let host_timestamp = get_host_timestamp(env)?;
            to_json_binary(&host_timestamp)?
        }
        QueryMsg::GetCommitmentPrefix {} => {
            let prefix = get_commitment_prefix();
            to_json_binary(&prefix)?
        }
        QueryMsg::CheckAndGetClient { client_id } => {
            let client = check_and_get_client(storage, &client_id)?;
            to_json_binary(&client)?
        }
    };

    Ok(result)
}

fn require_owner_caller(storage: &dyn Storage, caller: &Addr) -> StdResult<()> {
    let owner = OWNER.load(storage)?;
    require!(owner == caller, "Only owner may call this endpoint");

    Ok(())
}
