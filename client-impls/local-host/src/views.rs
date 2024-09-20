use client_common::{ClientStatus, GetLatestInfoResultType};
use common_types::{channel_types::height, ClientId, Hash, Timestamp};
use host::{host_views::ProxyTrait as _, storage::ProxyTrait as _};

use crate::local_host_types::{client_state, consensus_state};

multiversx_sc::imports!();

static DEFAULT_PROOF_BYTES: &[u8] = b"01";

#[multiversx_sc::module]
pub trait ViewsModule:
    client_common::CommonClientLogicModule
    + crate::client_logic::ClientLogicModule
    + common_modules::utils::UtilsModule
{
    /// Always returns the current block timestamp
    ///
    /// The timestamp is nanoseconds since unix epoch
    #[view(getTimestampAtHeight)]
    fn get_timestamp_at_height(
        &self,
        client_id: &ClientId<Self::Api>,
        height: &height::Data,
    ) -> Timestamp {
        self.require_valid_client_id(client_id);
        require!(height.revision_number == 0, "Invalid revision number");

        let current_block = self.blockchain().get_block_nonce();
        require!(
            height.revision_height <= current_block,
            "Invalid revision height"
        );

        let block_timestamp = self.blockchain().get_block_timestamp();
        self.checked_timestamp_to_unix_mul(block_timestamp)
    }

    /// Always returns the current block height
    #[view(getLatestHeight)]
    fn get_latest_height(&self, client_id: &ClientId<Self::Api>) -> height::Data {
        self.require_valid_client_id(client_id);

        height::Data {
            revision_number: 0,
            revision_height: self.blockchain().get_block_nonce(),
        }
    }

    /// Always returns "Active"
    #[view(getStatus)]
    fn get_status(&self, client_id: &ClientId<Self::Api>) -> ClientStatus {
        self.require_valid_client_id(client_id);

        ClientStatus::Active
    }

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

    /// A generic proof verification method which verifies a proof of the existence of a value at a given CommitmentPath at the specified height
    ///
    /// The caller is expected to construct the full CommitmentPath from a CommitmentPrefix and a standardized path (as defined in ICS 24)
    ///
    /// Proof uses "DEFAULT_PROOF_BYTES"
    #[view(verifyMembership)]
    fn verify_membership(
        &self,
        client_id: ClientId<Self::Api>,
        height: height::Data,
        _delay_time_period: Timestamp,
        _delay_block_period: u64,
        proof: Hash<Self::Api>,
        prefix: ManagedBuffer,
        path: ManagedBuffer,
        value: ManagedBuffer,
    ) -> bool {
        self.require_valid_client_id(&client_id);
        let _ = self.get_timestamp_at_height(&client_id, &height);
        self.require_ibc_prefix(&prefix);

        let default_proof = self
            .crypto()
            .keccak256(ManagedBuffer::from(DEFAULT_PROOF_BYTES));
        require!(proof == default_proof, "Invalid proof");

        let ibc_handler = self.ibc_handler().get();
        let hashed_path = self.crypto().keccak256(path);
        let hash: Hash<Self::Api> = self
            .host_proxy(ibc_handler)
            .get_commitment(&hashed_path)
            .execute_on_dest_context();

        hash == self.crypto().keccak256(value)
    }

    /*
    function verifyNonMembership(
        string memory clientId,
        Height.Data memory proofHeight,
        uint64,
        uint64,
        bytes memory proof,
        bytes memory prefix,
        bytes memory path
    ) public view returns (bool) {
        if (keccak256(abi.encodePacked(clientId)) != keccak256(abi.encodePacked(LocalhostClientLib.CLIENT_ID))) {
            revert InvalidClientID();
        }
        if (proofHeight.revision_number != 0) {
            revert InvalidHeightRevisionNumber();
        }
        if (proofHeight.revision_height > block.number) {
            revert InvalidHeightRevisionHeight();
        }
        if (keccak256(proof) != keccak256(LocalhostClientLib.sentinelProof())) {
            revert InvalidProof();
        }
        if (keccak256(IIBCHandler(ibcHandler).getCommitmentPrefix()) != keccak256(prefix)) {
            revert InvalidPrefix();
        }
        return IIBCHandler(ibcHandler).getCommitment(keccak256(path)) == bytes32(0);
    }

     */

    /// A generic proof verification method which verifies the absence of a given CommitmentPath at a specified height
    ///
    /// The caller is expected to construct the full CommitmentPath from a CommitmentPrefix and a standardized path (as defined in ICS 24)
    #[view(verifyNonMembership)]
    fn verify_non_membership(
        &self,
        client_id: ClientId<Self::Api>,
        height: height::Data,
        _delay_time_period: Timestamp,
        _delay_block_period: u64,
        proof: Hash<Self::Api>,
        prefix: ManagedBuffer,
        path: ManagedBuffer,
    ) -> bool {
        let _ = self.get_timestamp_at_height(&client_id, &height);
        self.require_ibc_prefix(&prefix);

        let default_proof = self
            .crypto()
            .keccak256(ManagedBuffer::from(DEFAULT_PROOF_BYTES));
        require!(proof == default_proof, "Invalid proof");

        let ibc_handler = self.ibc_handler().get();
        let hashed_path = self.crypto().keccak256(path);
        let hash: Hash<Self::Api> = self
            .host_proxy(ibc_handler)
            .get_commitment(&hashed_path)
            .execute_on_dest_context();

        hash == self.crypto().keccak256(ManagedBuffer::new())
    }

    /// returns the clientState corresponding to `clientId`
    #[view(getClientState)]
    fn get_client_state(&self, client_id: &ClientId<Self::Api>) -> client_state::Data {
        self.require_valid_client_id(client_id);

        let current_block = self.blockchain().get_block_nonce();
        client_state::Data {
            latest_height: height::Data::new(0, current_block),
        }
    }

    /// always returns the sentinel consensus state (i.e. '0')
    #[view(getConsensusState)]
    fn get_consensus_state(
        &self,
        client_id: &ClientId<Self::Api>,
        _height: &height::Data,
    ) -> consensus_state::Data {
        self.require_valid_client_id(client_id);

        consensus_state::Data { timestamp: 0 }
    }

    fn require_ibc_prefix(&self, prefix: &ManagedBuffer) {
        let ibc_handler = self.ibc_handler().get();
        let ibc_prefix: ManagedBuffer = self
            .host_proxy(ibc_handler)
            .get_commitment_prefix()
            .execute_on_dest_context();
        require!(prefix == &ibc_prefix, "Invalid prefix");
    }

    #[proxy]
    fn host_proxy(&self, sc_address: ManagedAddress) -> host::Proxy<Self::Api>;
}
