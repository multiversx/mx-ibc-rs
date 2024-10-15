pub mod merkle_prefix {
    use cosmwasm_schema::cw_serde;

    #[cw_serde]
    pub struct Data {
        pub key_prefix: Vec<u8>,
    }
}

pub mod connection_end {
    use cosmwasm_schema::cw_serde;

    use crate::{ClientId, UnixTimestamp, VersionVec};

    use super::counterparty;

    #[cw_serde]
    pub enum State {
        UninitializedUnspecified,
        Init,
        TryOpen,
        Open,
    }

    #[cw_serde]
    pub struct Data {
        pub client_id: ClientId,
        pub versions: VersionVec,
        pub state: State,
        pub counterparty: counterparty::Data,
        pub delay_period: UnixTimestamp,
    }
}

pub mod counterparty {
    use cosmwasm_schema::cw_serde;

    use crate::{ClientId, ConnectionId};

    use super::merkle_prefix;

    #[cw_serde]
    pub struct Data {
        pub client_id: ClientId,
        pub connection_id: ConnectionId,
        pub prefix: merkle_prefix::Data,
    }
}

pub mod version {
    use cosmwasm_schema::cw_serde;

    use crate::{FeatureId, FeatureVec};

    #[cw_serde]
    pub struct Data {
        pub identifier: FeatureId,
        pub features: FeatureVec,
    }
}
