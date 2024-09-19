use common_types::ClientId;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/*
struct MsgCreateClient {
        string clientType;
        bytes protoClientState;
        bytes protoConsensusState;
    }

    struct MsgUpdateClient {
        string clientId;
        bytes protoClientMessage;
    }
*/

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct MsgCreateClient<M: ManagedTypeApi> {
    pub client_id: ClientId<M>,
    // TODO
}

#[multiversx_sc::module]
pub trait UpdateClientsModule {

}