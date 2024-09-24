use common_types::{
    channel_types::height, ChannelId, Hash, PortId, Sequence, Timestamp, HASH_LENGTH,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const EMPTY_HASH: &[u8; HASH_LENGTH] = &[0u8; HASH_LENGTH];

#[derive(TopEncode)]
pub enum PacketReceipt {
    None,
    Successful,
}

/// `Packet` defines a type that carries data across different chains through IBC.
///
/// `sequence` corresponds to the order of sends and receives, where a packet with an earlier sequence number must be sent and received before a packet with a later sequence number
///
/// `source_port` identifies the port on the sending chain
///
/// `source_channel` identifies the channel end on the sending chain
///
/// `dest_port` identifies the port on the receiving chain
///
/// `dest_channel` identifies the channel end on the receiving chain
///
/// `data` is an opaque value which can be defined by the application logic of the associated modules
///
/// `timeout_height` indicates a consensus height on the destination chain after which the packet will no longer be processed, and will instead count as having timed-out
///
/// `timeout_timestamp` indicates a timestamp on the destination chain after which the packet will no longer be processed, and will instead count as having timed-out
pub struct Packet<M: ManagedTypeApi> {
    pub sequence: Sequence,
    pub source_port: PortId<M>,
    pub source_channel: ChannelId<M>,
    pub dest_port: PortId<M>,
    pub dest_channel: ChannelId<M>,
    pub data: ManagedBuffer<M>,
    pub timemout_height: height::Data,
    pub timeout_timestamp: Timestamp,
}

#[multiversx_sc::module]
pub trait IbcChannelLibModule: common_modules::utils::UtilsModule {
    fn receipt_commitment_to_receipt(&self, commitment: &Hash<Self::Api>) -> PacketReceipt {
        if commitment == &Hash::new_from_bytes(EMPTY_HASH) {
            return PacketReceipt::None;
        }

        let encoded_success = self.encode_to_buffer(&PacketReceipt::Successful);
        let successful_hash = self.crypto().keccak256(&encoded_success);
        if commitment == &successful_hash {
            return PacketReceipt::Successful;
        }

        sc_panic!("Unknown channel packet receipt commitment");
    }
}
