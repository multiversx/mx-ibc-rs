pub mod channel {
    use cosmwasm_schema::cw_serde;

    use crate::{channel_types::channel_counterparty, ConnectionHops, Sequence, Version};

    pub static ORDERED: &[u8] = b"ORDER_ORDERED";
    pub static UNORDERED: &[u8] = b"ORDER_UNORDERED";

    #[cw_serde]
    pub enum State {
        UninitializedUnspecified,
        Init,
        TryOpen,
        Open,
        Closed,
        Flushing,
        FlushComplete,
    }

    #[cw_serde]
    pub enum Order {
        NoneUnspecified,
        Unordered,
        Ordered,
    }

    impl Order {
        pub fn to_byte_slice(&self) -> &[u8] {
            match *self {
                Order::NoneUnspecified => {
                    panic!("Unknown channel order")
                }
                Order::Unordered => UNORDERED,
                Order::Ordered => ORDERED,
            }
        }
    }

    #[cw_serde]
    pub struct Data {
        pub state: State,
        pub ordering: Order,
        pub counterparty: channel_counterparty::Data,
        pub connection_hops: ConnectionHops,
        pub version: Version,
        pub upgrade_sequence: Sequence,
    }
}

pub mod channel_counterparty {
    use cosmwasm_schema::cw_serde;

    use crate::{ChannelId, PortId};

    #[cw_serde]
    pub struct Data {
        pub port_id: PortId,
        pub channel_id: ChannelId,
    }
}

pub mod height {
    use cosmwasm_schema::cw_serde;

    const U64_BYTES: usize = 8;

    #[cw_serde]
    #[derive(PartialOrd, Copy)]
    pub struct Data {
        pub revision_number: u64,
        pub revision_height: u64,
    }

    impl Data {
        pub fn is_zero(&self) -> bool {
            self.revision_number == 0 && self.revision_height == 0
        }

        pub fn to_u128(&self) -> u128 {
            let mut return_value = self.revision_number as u128;
            return_value <<= U64_BYTES;
            return_value |= self.revision_height as u128;

            return_value
        }
    }

    #[cfg(test)]
    mod tests {
        use core::cmp::Ordering;

        use super::*;

        impl Data {
            fn new(revision_number: u64, revision_height: u64) -> Self {
                Self {
                    revision_number,
                    revision_height,
                }
            }
        }

        #[test]
        fn partial_ord_test() {
            assert_eq!(
                Data::new(0, 100).partial_cmp(&Data::new(1, 50)),
                Some(Ordering::Less)
            );
            assert_eq!(
                Data::new(0, 100).partial_cmp(&Data::new(0, 100)),
                Some(Ordering::Equal)
            );
            assert_eq!(
                Data::new(0, 100).partial_cmp(&Data::new(0, 50)),
                Some(Ordering::Greater)
            );
            assert_eq!(
                Data::new(0, 50).partial_cmp(&Data::new(1, 50)),
                Some(Ordering::Less)
            );
        }
    }
}

pub mod timeout {
    use cosmwasm_schema::cw_serde;

    use crate::channel_types::height;
    use crate::UnixTimestamp;

    #[cw_serde]
    pub struct Data {
        pub height: height::Data,
        pub timestamp: UnixTimestamp,
    }
}

pub mod upgrade {
    use cosmwasm_schema::cw_serde;

    use crate::Sequence;

    use super::{timeout, upgrade_fields};

    #[cw_serde]
    pub struct Data {
        pub fields: upgrade_fields::Data,
        pub timeout: timeout::Data,
        pub next_sequence_send: Sequence,
    }
}

pub mod upgrade_fields {
    use cosmwasm_schema::cw_serde;

    use crate::{ConnectionHops, Version};

    use super::channel;

    #[cw_serde]
    pub struct Data {
        pub ordering: channel::Order,
        pub connection_hops: ConnectionHops,
        pub version: Version,
    }
}

pub mod error_receipt {
    use cosmwasm_schema::cw_serde;

    use crate::Sequence;

    #[cw_serde]
    pub struct Data {
        pub sequence: Sequence,
        pub message: Vec<u8>,
    }
}
