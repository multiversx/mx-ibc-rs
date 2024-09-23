pub mod merkle_prefix {
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub key_prefix: ManagedBuffer<M>,
    }
}

pub mod connection_end {
    use crate::ClientId;

    use super::{counterparty, version};

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
    pub enum State {
        StateUninitializedUnspecified,
        StateInit,
        StateTryOpen,
        StateOpen,
    }

    #[derive(TypeAbi, TopEncode, TopDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub client_id: ClientId<M>,
        pub versions: ManagedVec<M, version::Data<M>>,
        pub state: State,
        pub counterparty: counterparty::Data<M>,
        pub delay_period: u64, // TODO: Probably a timestamp
    }
}

pub mod counterparty {
    use crate::{ClientId, ConnectionId};

    use super::merkle_prefix;

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub client_id: ClientId<M>,
        pub connection_id: ConnectionId<M>,
        pub prefix: merkle_prefix::Data<M>,
    }
}

pub mod version {
    use crate::{FeatureId, FeatureVec};

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
    pub struct Data<M: ManagedTypeApi> {
        pub identifier: FeatureId<M>,
        pub features: FeatureVec<M>,
    }
}
