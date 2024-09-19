use common_types::{
    channel_types::height,
    qbft_types::{client_state, consensus_state, header},
    ClientId, Hash, Timestamp,
};

multiversx_sc::imports!();

// Likely don't need all this

// static HEADER_TYPE_URL: &[u8] = b"/ibc.lightclients.qbft.v1.Header";
// static CLIENT_STATE_TYPE_URL: &[u8] = b"/ibc.lightclients.qbft.v1.ClientState";
// static CONSENSUS_STATE_TYPE_URL: &[u8] = b"/ibc.lightclients.qbft.v1.ConsensusState";

pub struct ParsedBesuHeader<M: ManagedTypeApi> {
    pub base: header::Data<M>,
    pub height: height::Data,
    pub state_root: Hash<M>,
    pub time: Timestamp,
    pub validators: ManagedVec<M, ManagedAddress<M>>, // TODO: Was RLPReader.RLPItem[]. Why?
}

#[multiversx_sc::module]
pub trait QbftModule: host::host_views::HostViewsModule + host::storage::StorageModule {
    #[endpoint(initializeClient)]
    fn initialize_client(
        &self,
        client_id: ClientId<Self::Api>,
        client_state: client_state::Data<Self::Api>,
        consensus_state: consensus_state::Data<Self::Api>,
    ) -> height::Data {
        self.require_ibc_handler_caller();
        require!(
            client_state.latest_height.revision_height != 0,
            "Invalid client state height"
        );
        require!(!consensus_state.validators.is_empty(), "Empty validators");

        self.client_states(&client_id).set(&client_state);
        self.consensus_states(&client_id, &client_state.latest_height.to_biguint_concat())
            .set(consensus_state);

        client_state.latest_height
    }

    /// Timestamp is nanoseconds since unix epoch
    #[view(getTimestampAtHeight)]
    fn get_timestamp_at_height(
        &self,
        client_id: ClientId<Self::Api>,
        height: height::Data,
    ) -> Timestamp {
        let mapper = self.consensus_states(&client_id, &height.to_biguint_concat());
        require!(!mapper.is_empty(), "Consensus state not found");

        let consensus_state = mapper.get();
        self.checked_timestamp_to_unix_mul(consensus_state.timestamp)
    }

    /// returns the latest height of the client state corresponding to `clientId`
    #[view(getLatestHeight)]
    fn get_latest_height(&self, client_id: ClientId<Self::Api>) -> height::Data {
        let mapper = self.client_states(&client_id);
        require!(!mapper.is_empty(), "Client state not found");

        let client_state = mapper.get();
        client_state.latest_height
    }

    fn require_ibc_handler_caller(&self) {
        let caller = self.blockchain().get_caller();
        let ibc_handler = self.ibc_handler().get();
        require!(
            caller == ibc_handler,
            "Only the IBC handler may call this endpoint"
        );
    }

    #[view(getIbcHandler)]
    #[storage_mapper("ibcHandler")]
    fn ibc_handler(&self) -> SingleValueMapper<ManagedAddress>;

    // TODO: Replace the generic "ManagedBuffer" and "BigUint" with something specific once I figure out what it means
    // Was this:
    /*
        mapping(string => ClientState.Data) internal clientStates;
        mapping(string => mapping(uint128 => ConsensusState.Data)) internal consensusStates;
        mapping(string => mapping(uint128 => uint256)) internal processedTimes;
        mapping(string => mapping(uint128 => uint256)) internal processedHeights;
    */

    #[storage_mapper("clientStates")]
    fn client_states(
        &self,
        client_id: &ClientId<Self::Api>,
    ) -> SingleValueMapper<client_state::Data<Self::Api>>;

    #[storage_mapper("consensusStates")]
    fn consensus_states(
        &self,
        client_id: &ClientId<Self::Api>,
        height: &BigUint,
    ) -> SingleValueMapper<consensus_state::Data<Self::Api>>;

    #[storage_mapper("processedTimes")]
    fn processed_times(
        &self,
        buffer: &ManagedBuffer,
        biguint: &BigUint,
    ) -> SingleValueMapper<BigUint>;

    #[storage_mapper("processedHeights")]
    fn processed_heights(
        &self,
        buffer: &ManagedBuffer,
        biguint: &BigUint,
    ) -> SingleValueMapper<BigUint>;
}
