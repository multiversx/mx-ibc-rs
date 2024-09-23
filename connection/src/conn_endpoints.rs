use common_types::{
    connection_types::{connection_end, version},
    ConnectionId, VersionVec,
};

use crate::conn_types::MsgConnectionOpenInit;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ConnectionEndpointsModule:
    crate::conn_lib::ConnectionLibModule
    + crate::events::EventsModule
    + host::commitment::CommitmentModule
    + host::host_config::HostConfigModule
    + host::host_views::HostViewsModule
    + host::module_manager::ModuleManagerModule
    + host::storage::StorageModule
    + common_modules::client_lib::ClientLibModule
    + common_modules::host_lib::HostLibModule
    + common_modules::utils::UtilsModule
{
    /// Initialises a connection attempt on chain A.
    ///
    /// The generated connection identifier is returned.
    #[endpoint(connectionOpenInit)]
    fn connection_open_init(
        &self,
        args: MsgConnectionOpenInit<Self::Api>,
    ) -> ConnectionId<Self::Api> {
        let connection_id = self.generate_connection_id();
        let connection_mapper = self.connection_info(&connection_id);
        require!(connection_mapper.is_empty(), "Connection already exists");

        // ensure the client exists
        let _ = self.check_and_get_client(&args.client_id);
        require!(
            !args.counterparty.connection_id.is_empty(),
            "Invalid counterparty connection ID"
        );

        let mut connection_info = connection_end::Data {
            client_id: args.client_id,
            counterparty: args.counterparty,
            delay_period: args.delay_period,
            state: connection_end::State::StateInit,
            versions: VersionVec::new(),
        };

        self.set_versions_after_init(args.version, &mut connection_info.versions);
        self.update_connection_commitment(&connection_id, &connection_info);
        connection_mapper.set(connection_info);

        self.generated_connection_id_event(&connection_id);

        connection_id
    }

    #[view(getCompatibleVersions)]
    fn get_compatible_versions(&self) -> VersionVec<Self::Api> {
        VersionVec::from_single_item(self.default_ibc_version())
    }

    fn set_versions_after_init(
        &self,
        args_version: version::Data<Self::Api>,
        output: &mut VersionVec<Self::Api>,
    ) {
        let compatible_versions = self.get_compatible_versions();
        if !args_version.features.is_empty() {
            require!(
                self.is_supported_version(&compatible_versions, &args_version),
                "Version not supported"
            );

            output.push(args_version);
        } else {
            self.set_supported_versions(compatible_versions, output);
        }
    }

    fn update_connection_commitment(
        &self,
        connection_id: &ConnectionId<Self::Api>,
        connection_info: &connection_end::Data<Self::Api>,
    ) {
        let connection_key = self.get_connection_commitment_key(connection_id);
        let mut encoded_connection = ManagedBuffer::new();
        let _ = connection_info.top_encode(&mut encoded_connection);
        let hashed_connection = self.crypto().keccak256(encoded_connection);

        self.commitments(&connection_key).set(&hashed_connection);
    }

    fn generate_connection_id(&self) -> ConnectionId<Self::Api> {
        let next_conn_seq = self.get_next_connection_seq();

        sc_format!("connection-{}", next_conn_seq)
    }
}
