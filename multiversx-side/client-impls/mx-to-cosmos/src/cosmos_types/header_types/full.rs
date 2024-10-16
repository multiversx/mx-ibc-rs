use super::parts::{Commit, CosmWasmHeaderPart};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopDecode)]
pub struct SignedCosmWasmHeaderFull<M: ManagedTypeApi> {
    pub header: CosmWasmHeaderPart<M>,
    pub commit: Commit<M>,
}
