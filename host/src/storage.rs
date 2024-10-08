use common_types::{
    channel_types::{channel, upgrade},
    connection_types::connection_end,
    ChannelId, ClientId, ConnectionId, FixedLengthBuffer, PortId, Sequence, Timestamp,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, Default)]
pub struct HostInfo {
    pub next_client_seq: Sequence,
    pub next_connection_seq: Sequence,
    pub next_channel_seq: Sequence,
    pub expected_time_per_block: Timestamp,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct RecvStartSequence {
    pub seq: Sequence,
    pub prev_seq: Sequence,
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ChannelInfo<M: ManagedTypeApi> {
    pub channel: channel::Data<M>,
    pub next_seq_send: Sequence,
    pub next_seq_recv: Sequence,
    pub next_seq_ack: Sequence,
    pub upgrade: upgrade::Data<M>,
    pub latest_error_rec_seq: Sequence,
    pub recv_start_seq: RecvStartSequence,
    pub ack_start_seq: Sequence,
}

#[multiversx_sc::module]
pub trait StorageModule {
    // TODO: Figure out what "something" means
    #[storage_mapper("commitments")]
    fn commitments(
        &self,
        something: &FixedLengthBuffer<Self::Api>,
    ) -> SingleValueMapper<FixedLengthBuffer<Self::Api>>;

    #[storage_mapper("clientReg")]
    fn client_registry(&self, client_id: &ClientId<Self::Api>)
        -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("portCap")]
    fn port_capabilities(&self, port_id: &PortId<Self::Api>) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("channelCap")]
    fn channel_capabilities(
        &self,
        port_id: &PortId<Self::Api>,
        channel_id: &ChannelId<Self::Api>,
    ) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("hostInfo")]
    fn host_info(&self) -> SingleValueMapper<HostInfo>;

    #[storage_mapper("connInfo")]
    fn connection_info(
        &self,
        connection_id: &ConnectionId<Self::Api>,
    ) -> SingleValueMapper<connection_end::Data<Self::Api>>;

    #[storage_mapper("channelInfo")]
    fn channel_info(
        &self,
        channel_id: &ChannelId<Self::Api>,
    ) -> SingleValueMapper<ChannelInfo<Self::Api>>;
}
