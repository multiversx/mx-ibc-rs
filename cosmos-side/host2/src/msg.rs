use common_types2::{ClientId, ClientType, Hash, PortId, UnixTimestamp};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

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
