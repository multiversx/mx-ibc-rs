use client_common::{ClientStatus, GetLatestInfoResultType};
use common_types::{channel_types::height, ClientId, Timestamp};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ViewsModule:
    client_common::CommonClientLogicModule + crate::client_logic::ClientLogicModule
{
    /// returns the timestamp of the consensus state at the given height
    ///
    /// The timestamp is nanoseconds since unix epoch
    #[view(getTimestampAtHeight)]
    fn get_timestamp_at_height(
        &self,
        client_id: &ClientId<Self::Api>,
        height: &height::Data,
    ) -> Timestamp {
        let mapper = self.consensus_states(client_id, &height.to_biguint_concat());
        require!(!mapper.is_empty(), "Consensus state not found");

        let consensus_state = mapper.get();
        consensus_state.timestamp
    }

    /// returns the latest height of the client state corresponding to `clientId`
    #[view(getLatestHeight)]
    fn get_latest_height(&self, client_id: &ClientId<Self::Api>) -> height::Data {
        let mapper = self.client_states(client_id);
        require!(!mapper.is_empty(), "Client state not found");

        let client_state = mapper.get();
        client_state.latest_height
    }

    /// returns the status of the client corresponding to `clientId`
    ///
    /// A client status of "None" means the client is unknown
    #[view(getStatus)]
    fn get_status(&self, client_id: &ClientId<Self::Api>) -> ClientStatus {
        self.statuses(client_id).get()
    }

    /// returns the latest height, the latest timestamp, and the status of the client corresponding to `clientId`
    #[view(getLatestInfo)]
    fn get_latest_info(&self, client_id: ClientId<Self::Api>) -> GetLatestInfoResultType {
        let latest_height = self.get_latest_height(&client_id);
        let latest_timestamp = self.get_timestamp_at_height(&client_id, &latest_height);
        let client_status = self.get_status(&client_id);

        GetLatestInfoResultType {
            latest_height,
            latest_timestamp,
            client_status,
        }
    }
}
