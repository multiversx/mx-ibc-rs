pub mod channel {
    use crate::{channel_types::channel_counterparty, Sequence};

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
    pub enum State {
        StateUninitializedUnspecified,
        StateInit,
        StateTryOpen,
        StateOpen,
        StateClosed,
        StateFlushing,
        StateFlushComplete,
    }

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
    pub enum Order {
        OrderNoneUnspecified,
        OrderUnordered,
        OrderOrdered,
    }

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub state: State,
        pub ordering: Order,
        pub counterparty: channel_counterparty::Data<M>,
        pub connection_hops: ManagedVec<M, ManagedBuffer<M>>, // TODO: Maybe custom type
        pub version: ManagedBuffer<M>,
        pub upgrade_sequence: Sequence,
    }
}

pub mod channel_counterparty {
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    use crate::{ChannelId, PortId};

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub port_id: PortId<M>,
        pub channel_id: ChannelId<M>,
    }
}

pub mod height {
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
    pub struct Data {
        pub revision_number: u64,
        pub revision_height: u64,
    }

    impl Data {
        pub fn to_biguint_concat<M: ManagedTypeApi>(&self) -> BigUint<M> {
            let mut buffer = ManagedBuffer::new();
            let _ = self.revision_number.dep_encode(&mut buffer);
            let _ = self.revision_height.dep_encode(&mut buffer);

            BigUint::from_bytes_be_buffer(&buffer)
        }
    }
}

pub mod timeout {
    use crate::channel_types::height;
    use crate::Timestamp;

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
    pub struct Data {
        pub height: height::Data,
        pub timestamp: Timestamp,
    }
}

pub mod upgrade {
    use crate::Sequence;

    use super::{timeout, upgrade_fields};

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub fields: upgrade_fields::Data<M>,
        pub timeout: timeout::Data,
        pub next_sequence_send: Sequence,
    }
}

pub mod upgrade_fields {
    use super::channel;

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub ordering: channel::Order,
        pub connection_hops: ManagedVec<M, ManagedBuffer<M>>, // TODO: Maybe custom type
        pub version: ManagedBuffer<M>,
    }
}

pub mod error_receipt {
    use crate::Sequence;

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode)]
    pub struct Data<M: ManagedTypeApi> {
        pub sequence: Sequence,
        pub message: ManagedBuffer<M>,
    }
}
