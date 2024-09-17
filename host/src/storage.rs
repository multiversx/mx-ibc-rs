use common_types::FixedLengthBuffer;

multiversx_sc::imports!();

/*
/// @custom:storage-location erc7201:ibc.host
    struct HostStorage {
        mapping(string => address) clientRegistry;
        mapping(string => address) portCapabilities;
        mapping(string => mapping(string => address)) channelCapabilities;
        uint64 nextClientSequence;
        uint64 nextConnectionSequence;
        uint64 nextChannelSequence;
        uint64 expectedTimePerBlock;
    }

    /// @custom:storage-location erc7201:ibc.client
    struct ClientStorage {
        string clientType;
        address clientImpl;
    }

    /// @custom:storage-location erc7201:ibc.connection
    struct ConnectionStorage {
        ConnectionEnd.Data connection;
    }

    struct RecvStartSequence {
        uint64 sequence;
        uint64 prevSequence;
    }

    /// @custom:storage-location erc7201:ibc.channel
    struct ChannelStorage {
        Channel.Data channel;
        uint64 nextSequenceSend;
        uint64 nextSequenceRecv;
        uint64 nextSequenceAck;
        Upgrade.Data upgrade;
        uint64 latestErrorReceiptSequence;
        RecvStartSequence recvStartSequence;
        uint64 ackStartSequence;
    }
*/

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("commitments")]
    fn commitments(
        &self,
        something: &FixedLengthBuffer<Self::Api>,
    ) -> SingleValueMapper<FixedLengthBuffer<Self::Api>>;
}
