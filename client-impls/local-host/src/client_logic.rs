use common_types::{channel_types::height, ClientId};

use crate::local_host_types::{client_state, consensus_state};

multiversx_sc::imports!();

// static CLIENT_TYPE: &[u8] = b"09-localhost";
static CLIENT_ID: &[u8] = b"09-localhost-0";

#[multiversx_sc::module]
pub trait ClientLogicModule: client_common::CommonClientLogicModule {
    /// initializes a new localhost client with the given client identifier, client state, and consensus state.
    ///
    /// `client_id`` the client identifier must be match with `CLIENT_ID`
    ///
    /// `client_state` the client state's latest height must be match with the current block number
    ///
    /// `consensus_state` the consensus state must be match with the sentinel consensus state (i.e. 0)
    #[endpoint(initializeClient)]
    fn initialize_client(
        &self,
        client_id: ClientId<Self::Api>,
        client_state: client_state::Data,
        consensus_state: consensus_state::Data,
    ) -> height::Data {
        self.require_ibc_handler_caller();
        self.require_valid_client_id(&client_id);
        require!(consensus_state.timestamp == 0, "Invalid consensus state");
        require!(
            client_state.latest_height.revision_number == 0,
            "Invalid revision number"
        );

        let current_block = self.blockchain().get_block_nonce();
        require!(
            client_state.latest_height.revision_height == current_block,
            "Invalid revision height"
        );

        height::Data {
            revision_number: 0,
            revision_height: current_block,
        }
    }

    /*
    /**
     * @dev updateClient updates the client state commitment with the current block number.
     * @param clientId the client identifier must be match with `LocalhostClientLib.CLIENT_ID`
     */
    function updateClient(string calldata clientId) public returns (Height.Data[] memory heights) {
        if (keccak256(abi.encodePacked(clientId)) != keccak256(abi.encodePacked(LocalhostClientLib.CLIENT_ID))) {
            revert InvalidClientID();
        }
        IIBCHandler(ibcHandler).updateClientCommitments(clientId, new Height.Data[](0));
        return heights;
    }
     */

    /// updates the client state commitment with the current block number
    ///
    /// `client_id`` the client identifier must be match with `CLIENT_ID`
    #[endpoint(updateClient)]
    fn update_client(&self, client_id: ClientId<Self::Api>) -> ManagedVec<height::Data> {
        self.require_valid_client_id(&client_id);

        let ibc_handler = self.ibc_handler().get();
        // TODO: Call ibc handler function

        ManagedVec::new()
    }

    fn require_valid_client_id(&self, client_id: &ClientId<Self::Api>) {
        require!(client_id == CLIENT_ID, "Invalid client ID");
    }
}
