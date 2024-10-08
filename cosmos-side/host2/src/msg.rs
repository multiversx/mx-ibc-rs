use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
}
