#[allow(clippy::module_inception)]
pub mod channel {
    use crate::channel::channel_counterparty;

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Default)]
    pub enum State {
        #[default]
        StateUninitializedUnspecified,
        StateInit,
        StateTryOpen,
        StateOpen,
        StateClosed,
        StateFlushing,
        StateFlushComplete,
    }

    impl State {
        pub fn is_uninitialized(&self) -> bool {
            matches!(self, State::StateUninitializedUnspecified)
        }
    }

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Default)]
    pub enum Order {
        #[default]
        OrderNoneUnspecified,
        OrderUnordered,
        OrderOrdered,
    }

    impl Order {
        pub fn is_unspecified(&self) -> bool {
            matches!(self, Order::OrderNoneUnspecified)
        }
    }

    #[derive(TypeAbi, TopEncode, TopDecode, Default)]
    pub struct Data<M: ManagedTypeApi> {
        pub state: State,
        pub ordering: Order,
        pub counterparty: channel_counterparty::Data<M>,
        pub connection_hops: ManagedVec<M, ManagedBuffer<M>>, // TODO: Maybe custom type
        pub version: ManagedBuffer<M>,
        pub upgrade_sequence: u64,
    }

    impl<M: ManagedTypeApi> Data<M> {
        pub fn is_empty(&self) -> bool {
            if !self.state.is_uninitialized() {
                return false;
            }

            if !self.ordering.is_unspecified() {
                return false;
            }

            if !self.connection_hops.is_empty() {
                return false;
            }

            if !self.version.is_empty() {
                return false;
            }

            if self.upgrade_sequence != 0 {
                return false;
            }

            true
        }
    }
}

pub mod channel_counterparty {
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    use crate::{ChannelId, PortId};

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Default)]
    pub struct Data<M: ManagedTypeApi> {
        pub port_id: PortId<M>,
        pub channel_id: ChannelId<M>,
    }

    impl<M: ManagedTypeApi> Data<M> {
        pub fn is_empty(&self) -> bool {
            if !self.port_id.is_empty() {
                return false;
            }

            if !self.channel_id.is_empty() {
                return false;
            }

            true
        }
    }
}

pub mod height {
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Default)]
    pub struct Data {
        pub revision_number: u64,
        pub revision_height: u64,
    }

    impl Data {
        pub fn is_empty(&self) -> bool {
            if self.revision_number != 0 {
                return false;
            }

            if self.revision_height != 0 {
                return false;
            }

            true
        }
    }
}

pub mod timeout {
    use crate::channel::height;
    use crate::Timestamp;

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Default)]
    pub struct Data {
        pub height: height::Data,
        pub timestamp: Timestamp,
    }

    impl Data {
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.timestamp == 0
        }
    }
}

pub mod upgrade {
    use super::{timeout, upgrade_fields};

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, Default)]
    pub struct Data<M: ManagedTypeApi> {
        pub fields: upgrade_fields::Data<M>,
        pub timeout: timeout::Data,
        pub next_sequence_send: u64,
    }

    impl<M: ManagedTypeApi> Data<M> {
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.next_sequence_send == 0
        }
    }
}

pub mod upgrade_fields {
    use super::channel;

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Default)]
    pub struct Data<M: ManagedTypeApi> {
        pub ordering: channel::Order,
        pub connection_hops: ManagedVec<M, ManagedBuffer<M>>, // TODO: Maybe custom type
        pub version: ManagedBuffer<M>,
    }

    impl<M: ManagedTypeApi> Data<M> {
        pub fn is_empty(&self) -> bool {
            if !self.ordering.is_unspecified() {
                return false;
            }

            if !self.connection_hops.is_empty() {
                return false;
            }

            if !self.version.is_empty() {
                return false;
            }

            true
        }
    }
}

pub mod error_receipt {
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, Default)]
    pub struct Data<M: ManagedTypeApi> {
        pub sequence: u64,
        pub message: ManagedBuffer<M>,
    }

    impl<M: ManagedTypeApi> Data<M> {
        pub fn is_empty(&self) -> bool {
            if self.sequence != 0 {
                return false;
            }

            if !self.message.is_empty() {
                return false;
            }

            true
        }
    }
}
