use common_types::ClientId;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("generatedClientIdEvent")]
    fn generated_client_id_event(&self, client_id: &ClientId<Self::Api>);
}
