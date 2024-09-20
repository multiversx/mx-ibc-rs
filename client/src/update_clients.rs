use common_types::{channel_types::height, ClientId, ClientType};
use host::storage::ClientInfo;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopDecode)]
pub struct MsgCreateClient<M: ManagedTypeApi> {
    pub client_type: ClientType<M>,
    pub encoded_client_state: ManagedBuffer<M>,
    pub encoded_consensus_state: ManagedBuffer<M>,
}

#[derive(TypeAbi, TopDecode)]
pub struct MsgUpdateClient<M: ManagedTypeApi> {
    pub client_id: ClientId<M>,
    pub encoded_client_message: ManagedBuffer<M>,
}

mod generic_client_proxy {
    use common_types::channel_types::height;

    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait GenericClientProxy {
        #[endpoint(initializeClient)]
        fn initialize_client(
            &self,
            encoded_client_state: ManagedBuffer,
            encoded_consensus_state: ManagedBuffer,
        ) -> height::Data;
    }
}

#[multiversx_sc::module]
pub trait UpdateClientsModule:
    crate::events::EventsModule
    + host::commitment::CommitmentModule
    + host::host_config::HostConfigModule
    + host::host_views::HostViewsModule
    + host::module_manager::ModuleManagerModule
    + host::storage::StorageModule
    + common_modules::client_lib::ClientLibModule
    + common_modules::host_lib::HostLibModule
    + common_modules::utils::UtilsModule
{
    /// creates a new client state and populates it with a given consensus state
    #[endpoint(createClient)]
    fn create_client(&self, args: MsgCreateClient<Self::Api>) -> ClientId<Self::Api> {
        let client_impl_mapper = self.client_registry(&args.client_type);
        require!(!client_impl_mapper.is_empty(), "Client not registered");

        // TODO: Check register function

        let client_impl = client_impl_mapper.get();
        let client_id = self.generate_client_identifier(&args.client_type);
        self.client_info(&client_id).set(ClientInfo {
            client_type: args.client_type,
            client_impl: client_impl.clone(),
        });

        let client_state_hash = self.crypto().keccak256(&args.encoded_client_state);
        let consensus_state_hash = self.crypto().keccak256(&args.encoded_consensus_state);
        let height: height::Data = self
            .generic_client_proxy_impl(client_impl)
            .initialize_client(args.encoded_client_state, args.encoded_consensus_state)
            .execute_on_dest_context();

        // update commitments
        let client_comm_key = self.get_client_state_commitment_key(&client_id);
        let consensus_comm_key = self.get_consensus_state_commitment_key(
            &client_id,
            height.revision_number,
            height.revision_height,
        );
        self.commitments(&client_comm_key).set(&client_state_hash);
        self.commitments(&consensus_comm_key)
            .set(&consensus_state_hash);

        self.generated_client_id_event(&client_id);

        client_id
    }

    fn generate_client_identifier(
        &self,
        client_type: &ClientType<Self::Api>,
    ) -> ClientId<Self::Api> {
        let next_client_seq = self.host_info().update(|host_info| {
            let returned_val = host_info.next_client_seq;
            host_info.next_client_seq += 1;

            returned_val
        });

        sc_format!("{}-{}", client_type, next_client_seq)
    }

    #[proxy]
    fn generic_client_proxy_impl(
        &self,
        sc_address: ManagedAddress,
    ) -> generic_client_proxy::GenericClientProxy<Self::Api>;
}
