use common_types2::{ClientType, PortId, UnixTimestamp};
use cosmwasm_schema::cw_serde;
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
