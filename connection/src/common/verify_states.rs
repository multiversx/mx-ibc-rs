use common_types::{
    connection_types::{connection_end, counterparty, merkle_prefix},
    ConnectionId, Hash,
};

use crate::conn_internal::{
    VerifyClientStateArgs, VerifyConnectionStateArgs, VerifyConsensusStateArgs,
};

use super::conn_types::MsgConnectionOpenTry;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait VerifyStatesModule:
    super::conn_lib::ConnectionLibModule
    + crate::conn_internal::ConnectionInternalModule
    + host::commitment::CommitmentModule
    + host::host_config::HostConfigModule
    + host::host_views::HostViewsModule
    + host::module_manager::ModuleManagerModule
    + host::storage::StorageModule
    + common_modules::client_lib::ClientLibModule
    + common_modules::host_lib::HostLibModule
    + common_modules::utils::UtilsModule
{
    fn verify_all_states_open_try(
        &self,
        connection_info: connection_end::Data<Self::Api>,
        self_consensus_state: &Hash<Self::Api>,
        args: MsgConnectionOpenTry<Self::Api>,
    ) {
        let expected_counterparty = counterparty::Data {
            client_id: args.client_id,
            connection_id: ConnectionId::new(),
            prefix: merkle_prefix::Data {
                key_prefix: self.get_commitment_prefix(),
            },
        };
        let expected_connection = connection_end::Data {
            client_id: args.counterparty.client_id,
            counterparty: expected_counterparty,
            state: connection_end::State::Init,
            delay_period: args.delay_period,
            versions: args.counterparty_versions,
        };

        self.verify_connection_state(VerifyConnectionStateArgs {
            connection_info: connection_info.clone(),
            height: args.proof_height,
            proof: args.proof_init,
            counterparty_connection_id: args.counterparty.connection_id,
            counterparty_connection_info: expected_connection,
        });
        self.verify_client_state(VerifyClientStateArgs {
            connection_info: connection_info.clone(),
            height: args.proof_height,
            path: self.get_client_state_path(&connection_info.counterparty.client_id),
            proof: args.proof_client,
            client_state_bytes: args.client_state_bytes,
        });
        self.verify_consensus_state(VerifyConsensusStateArgs {
            connection_info,
            height: args.proof_height,
            consensus_height: args.consensus_height,
            proof: args.proof_consensus,
            consensus_state_bytes: self_consensus_state.as_managed_buffer().clone(),
        });
    }
}
