pub mod merkle_prefix {
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Default)]
    pub struct Data<M: ManagedTypeApi> {
        pub key_prefix: ManagedBuffer<M>,
    }

    impl<M: ManagedTypeApi> Data<M> {
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.key_prefix.is_empty()
        }
    }
}

pub mod connection_end {
    use crate::ClientId;

    use super::{counterparty, version};

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Default)]
    pub enum State {
        #[default]
        StateUninitializedUnspecified,
        StateInit,
        StateTryOpen,
        StateOpen,
    }

    impl State {
        pub fn is_uninitialized(&self) -> bool {
            matches!(self, State::StateUninitializedUnspecified)
        }
    }

    #[derive(TypeAbi, TopEncode, TopDecode, Default)]
    pub struct Data<M: ManagedTypeApi> {
        pub client_id: ClientId<M>,
        pub versions: ManagedVec<M, version::Data<M>>,
        pub state: State,
        pub counterparty: counterparty::Data<M>,
        pub delay_period: u64, // TODO: Probably a timestamp
    }

    impl<M: ManagedTypeApi> Data<M> {
        pub fn is_empty(&self) -> bool {
            if !self.client_id.is_empty() {
                return false;
            }

            if !self.versions.is_empty() {
                return false;
            }

            if !self.state.is_uninitialized() {
                return false;
            }

            if self.delay_period != 0 {
                return false;
            }

            true
        }
    }
}

pub mod counterparty {
    use crate::{ClientId, ConnectionId};

    use super::merkle_prefix;

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Default)]
    pub struct Data<M: ManagedTypeApi> {
        pub client_id: ClientId<M>,
        pub connection_id: ConnectionId<M>,
        pub prefix: merkle_prefix::Data<M>,
    }

    impl<M: ManagedTypeApi> Data<M> {
        pub fn is_empty(&self) -> bool {
            if !self.client_id.is_empty() {
                return false;
            }

            if !self.connection_id.is_empty() {
                return false;
            }

            true
        }
    }
}

pub mod version {
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(
        TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Default, ManagedVecItem,
    )]
    pub struct Data<M: ManagedTypeApi> {
        pub identifier: ManagedBuffer<M>,
        pub features: ManagedVec<M, ManagedBuffer<M>>,
    }

    impl<M: ManagedTypeApi> Data<M> {
        pub fn is_empty(&self) -> bool {
            if !self.identifier.is_empty() {
                return false;
            }

            if !self.features.is_empty() {
                return false;
            }

            true
        }
    }
}
