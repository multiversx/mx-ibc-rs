use common_types::{ClientId, Timestamp};

multiversx_sc::imports!();

static DEFAULT_COMMITMENT_PREFIX: &[u8] = b"ibc";
const NANO_SECONDS_MULT: u64 = 1_000_000_000;

#[multiversx_sc::module]
pub trait HostViewsModule: crate::storage::StorageModule {
    /// Returns the current timestamp (Unix time in nanoseconds) of the host chain.
    #[view(getHostTimestamp)]
    fn get_host_timestamp(&self) -> Timestamp {
        let block_timestamp = self.blockchain().get_block_timestamp();

        self.checked_timestamp_to_unix_mul(block_timestamp)
    }

    #[view(getCommitmentPrefix)]
    fn get_commitment_prefix(&self) -> ManagedBuffer {
        ManagedBuffer::from(DEFAULT_COMMITMENT_PREFIX)
    }

    #[view(checkAndGetClient)]
    fn check_and_get_client(&self, client_id: ClientId<Self::Api>) -> ManagedAddress {
        let mapper = self.client_registry(&client_id);
        require!(!mapper.is_empty(), "Client not found");

        mapper.get()
    }

    fn checked_timestamp_to_unix_mul(&self, timestamp: Timestamp) -> Timestamp {
        match timestamp.checked_mul(NANO_SECONDS_MULT) {
            Some(result) => result,
            None => sc_panic!("Overlow!!!"),
        }
    }
}
