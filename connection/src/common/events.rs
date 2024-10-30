use common_types::ConnectionId;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("generatedConnectionIdentifier")]
    fn generated_connection_id_event(&self, connection_id: &ConnectionId<Self::Api>);
}
